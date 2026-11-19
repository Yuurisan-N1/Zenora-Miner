use crate::api::endpoint::MINING_STATUS;
use crate::domain::account::Account;
use crate::domain::mining::MiningState;
use crate::support::log::lr;

pub async fn read(account: &Account) -> Option<MiningState> {
    let reply = account.get(MINING_STATUS, &[]).await;
    if reply.status != 200 {
        lr("The mining status could not be read from the server");
        return None;
    }
    Some(MiningState::parse(&reply.body))
}
