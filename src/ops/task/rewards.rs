use crate::api::endpoint::{reward_claim, reward_complete, REWARDS};
use crate::domain::account::Account;
use crate::domain::task::Campaign;
use crate::support::consts::net::GATE_OFF;
use crate::support::log::{lg, lr, ly};

pub async fn claim_all(account: &Account) {
    let reply = account.get(REWARDS, &[]).await;
    if reply.status != 200 {
        lr("The reward list could not be read from the server");
        return;
    }
    let campaigns = Campaign::list(&reply.body);
    let mut claimed = 0usize;
    let mut manual = 0usize;

    for campaign in campaigns.iter().filter(|item| item.available) {
        let title = campaign.slug_or();
        if campaign.kind == "airdrop" || campaign.kind == "giveaway" {
            let body = account
                .post(&reward_claim(&campaign.id), serde_json::json!({}), &[])
                .await;
            if body.status == 200 {
                claimed += 1;
                let paid = body
                    .body
                    .get("paid")
                    .map(|value| value.to_string())
                    .unwrap_or_else(|| "0".to_string());
                lg(&format!("Reward {title} was claimed with {paid} ZNR"));
            } else {
                let reason = crate::app::bootstrap::clean_text(
                    body.body
                        .get("error")
                        .or_else(|| body.body.get("message"))
                        .and_then(|value| value.as_str())
                        .unwrap_or("refused"),
                    "refused",
                );
                lr(&format!("Reward {title} was refused with {reason}"));
            }
        } else if campaign.kind == "task" && campaign.verification == "ad_gated" {
            let token = crate::ops::task::ads::ad_token(account, "task", Some(&campaign.id)).await;
            let payload = match token {
                Some(value) if value == GATE_OFF => {
                    serde_json::json!({ "proof": { "done": true } })
                }
                Some(value) => {
                    serde_json::json!({ "proof": { "done": true }, "adToken": value })
                }
                None => {
                    lr(&format!(
                        "The advertisement for reward {title} could not be completed"
                    ));
                    continue;
                }
            };
            let body = account
                .post(&reward_complete(&campaign.id), payload, &[])
                .await;
            if body.status == 200 {
                claimed += 1;
                let paid = body
                    .body
                    .get("paid")
                    .map(|value| value.to_string())
                    .unwrap_or_else(|| "0".to_string());
                lg(&format!("Reward {title} was claimed with {paid} ZNR"));
            } else {
                let reason = crate::app::bootstrap::clean_text(
                    body.body
                        .get("error")
                        .or_else(|| body.body.get("message"))
                        .and_then(|value| value.as_str())
                        .unwrap_or("refused"),
                    "refused",
                );
                lr(&format!("Reward {title} was refused with {reason}"));
            }
        } else {
            manual += 1;
        }
    }

    if claimed == 0 && manual == 0 {
        ly("Every available reward was already claimed on this account");
    } else if manual > 0 {
        ly(&format!(
            "{manual} rewards still wait for a manual proof of work"
        ));
    }
}
