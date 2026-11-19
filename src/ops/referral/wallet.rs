use crate::api::endpoint::WALLET;
use crate::domain::account::Account;
use crate::support::log::{lg, lr};

pub async fn report(account: &Account) {
    let reply = account.get(WALLET, &[]).await;
    if reply.status != 200 {
        lr("The wallet could not be read from the server");
        return;
    }
    let balance = reply
        .body
        .get("zenora")
        .and_then(|item| item.get("balance"))
        .map(|value| value.to_string())
        .unwrap_or_else(|| "0".to_string());
    let address = reply
        .body
        .get("address")
        .and_then(|value| value.as_str())
        .unwrap_or("unknown");
    lg(&format!("Wallet holds {balance} ZNR on address {address}"));
}
