use crate::api::endpoint::MINING_CLAIM;
use crate::domain::account::Account;
use crate::support::consts::net::GATE_OFF;
use crate::support::log::{lg, lr};

pub async fn claim(account: &Account, session_id: &str) -> bool {
    let token = match crate::ops::task::ads::ad_token(account, "claim", None).await {
        Some(value) => value,
        None => {
            lr("The claim advertisement could not be completed");
            return false;
        }
    };
    let mut body = if token == GATE_OFF {
        serde_json::json!({ "sessionId": session_id })
    } else {
        serde_json::json!({ "adToken": token, "sessionId": session_id })
    };
    let signed = match crate::ops::auth::passkey::assertion(account).await {
        Some(value) => value,
        None => {
            lr("The passkey challenge could not be signed for this claim");
            return false;
        }
    };
    let (mut assertion, entry) = signed;
    if let Some(handle) = entry.user_handle.as_deref() {
        if let Some(response) = assertion
            .get_mut("response")
            .and_then(|value| value.as_object_mut())
        {
            response.insert(
                "userHandle".to_string(),
                serde_json::Value::String(handle.to_string()),
            );
        }
    }
    if let Some(map) = body.as_object_mut() {
        map.insert("passkey".to_string(), assertion);
    }
    let headers = crate::ops::auth::login::dashboard_headers();
    let reply = account.post(MINING_CLAIM, body, &headers).await;
    if reply.status == 200 {
        let amount = reply
            .body
            .get("claimed")
            .and_then(|claimed| claimed.get("amount"))
            .map(|value| value.to_string())
            .unwrap_or_else(|| "0".to_string());
        lg(&format!("Mining session was claimed with {amount} ZNR"));
        true
    } else {
        let reason = crate::app::bootstrap::clean_text(
            reply
                .body
                .get("error")
                .or_else(|| reply.body.get("message"))
                .and_then(|value| value.as_str())
                .unwrap_or("refused"),
            "refused",
        );
        lr(&format!("Mining claim was refused with {reason}"));
        false
    }
}

pub async fn claim_ready(account: &Account, sessions: &[String]) {
    for session_id in sessions {
        claim(account, session_id).await;
    }
}
