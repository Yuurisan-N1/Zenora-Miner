use crate::api::endpoint::MINING_START;
use crate::domain::account::Account;
use crate::support::log::{lg, lr, ly};

pub async fn ensure_running(account: &Account, state: &str) -> bool {
    if state != "idle" {
        ly("A mining block is already running on this account");
        return false;
    }
    let reply = account.post(MINING_START, serde_json::json!({}), &[]).await;
    if reply.status == 200 {
        lg("A fresh mining block was started for this account");
        true
    } else {
        lr("The mining block could not be started");
        false
    }
}
