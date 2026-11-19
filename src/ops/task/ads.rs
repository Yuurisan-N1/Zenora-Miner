use crate::api::endpoint::{ad_select, ADS_COMPLETE, ADS_START};
use crate::domain::account::Account;
use crate::support::consts::net::GATE_OFF;
use std::time::Duration;

pub async fn ad_token(
    account: &Account,
    placement: &str,
    reference: Option<&str>,
) -> Option<String> {
    let mut payload = serde_json::json!({ "placement": placement, "locale": "en" });
    if let Some(value) = reference {
        if let Some(map) = payload.as_object_mut() {
            map.insert(
                "ref".to_string(),
                serde_json::Value::String(value.to_string()),
            );
        }
    }
    let start = account.post(ADS_START, payload, &[]).await;
    if start.status != 200 {
        if std::env::var("ZM_DEBUG").is_ok() {
            crate::support::log::lr(&format!(
                "ads/start answered {} with {}",
                start.status, start.body
            ));
        }
        return None;
    }
    let ad_id = start
        .body
        .get("adId")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let candidates = start
        .body
        .get("candidates")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    if ad_id.is_empty() || candidates.is_empty() {
        return Some(GATE_OFF.to_string());
    }
    let candidate = candidates
        .iter()
        .find(|item| {
            item.get("adapter")
                .and_then(|v| v.as_str())
                .unwrap_or("network")
                != "network"
        })
        .or_else(|| candidates.first())
        .cloned()
        .unwrap_or(serde_json::json!({}));

    let mut select_body = serde_json::json!({
        "source": candidate.get("source").cloned().unwrap_or(serde_json::Value::Null),
    });
    if let Some(creative) = candidate.get("creative") {
        if let Some(map) = select_body.as_object_mut() {
            map.insert(
                "creativeId".to_string(),
                creative
                    .get("id")
                    .cloned()
                    .unwrap_or(serde_json::Value::Null),
            );
        }
    }
    let selected = account.post(&ad_select(&ad_id), select_body, &[]).await;
    if selected.status != 200 {
        if std::env::var("ZM_DEBUG").is_ok() {
            crate::support::log::lr(&format!(
                "ads/{ad_id}/select answered {} with {}",
                selected.status, selected.body
            ));
        }
        return None;
    }
    let seconds = selected
        .body
        .get("minSeconds")
        .or_else(|| candidate.get("minSeconds"))
        .and_then(|value| match value {
            serde_json::Value::Number(raw) => raw.as_f64(),
            serde_json::Value::String(raw) => raw.parse::<f64>().ok(),
            _ => None,
        })
        .map(|value| value as u64)
        .unwrap_or(5);
    tokio::time::sleep(Duration::from_secs(seconds)).await;

    let proof = serde_json::json!({
        "nonce": candidate.get("nonce").cloned().unwrap_or(serde_json::Value::Null),
        "watchedSeconds": seconds,
    });
    let done = account
        .post(
            ADS_COMPLETE,
            serde_json::json!({ "adId": ad_id, "proof": proof }),
            &[],
        )
        .await;
    if done.status != 200 {
        if std::env::var("ZM_DEBUG").is_ok() {
            crate::support::log::lr(&format!(
                "ads/complete answered {} with {}",
                done.status, done.body
            ));
        }
        return None;
    }
    done.body
        .get("token")
        .and_then(|v| v.as_str())
        .map(|value| value.to_string())
}
