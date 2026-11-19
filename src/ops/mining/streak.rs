use crate::api::endpoint::MINING_STREAK;
use crate::domain::account::Account;
use crate::support::log::{lg, lr, ly};

pub async fn count(account: &Account, counted_today: bool, days: i64) {
    if counted_today {
        ly("Daily streak was already counted today");
        return;
    }
    let reply = account
        .post(MINING_STREAK, serde_json::json!({}), &[])
        .await;
    match reply.status {
        200 => {
            let reported = reply
                .body
                .get("streak")
                .and_then(|streak| streak.get("days"))
                .and_then(|value| value.as_i64())
                .unwrap_or(days);
            lg(&format!("Daily streak was counted for day {reported}"));
        }
        409 => ly("The daily streak is not ready for this account yet"),
        _ => lr("The daily streak was refused by the server"),
    }
}
