use crate::api::endpoint::{login_page, SITE_KEY};
use crate::domain::account::{Account, AccountLine};
use crate::net::solver::Gate;
use crate::support::log::{gap, lg, lr, ly, mask_proxy};
use std::time::Duration;

pub async fn exit_country(proxy: Option<&str>) -> String {
    let mut builder = reqwest::Client::builder().timeout(Duration::from_secs(10));
    if let Some(raw) = proxy {
        if let Ok(entry) = reqwest::Proxy::all(raw) {
            builder = builder.proxy(entry);
        }
    }
    let client = match builder.build() {
        Ok(client) => client,
        Err(_) => return String::new(),
    };
    match client.get("https://ipinfo.io/json").send().await {
        Ok(reply) => match reply.json::<serde_json::Value>().await {
            Ok(body) => body
                .get("country")
                .and_then(|value| value.as_str())
                .unwrap_or("")
                .to_uppercase(),
            Err(_) => String::new(),
        },
        Err(_) => String::new(),
    }
}

pub async fn run_account(line: AccountLine, solver_url: &str) {
    let account = match Account::open(line) {
        Ok(account) => account,
        Err(message) => {
            lr(&format!("The account could not be prepared: {message}"));
            return;
        }
    };
    let path = account.path();
    if let Some(entry) = account.line.entry() {
        ly(&format!("Using proxy {}", mask_proxy(entry.as_str())));
    }

    let local = exit_country(None).await;
    if local == "VN" {
        lr("Vietnam is blocked by the bot maker");
        return;
    }
    let remote = exit_country(account.line.proxy.as_deref()).await;
    if remote == "VN" {
        lr(&format!(
            "Vietnam is blocked by the proxy exit for account on line {path}"
        ));
        return;
    }

    let mut gate = match Gate::new(solver_url) {
        Ok(gate) => gate,
        Err(message) => {
            lr(&message);
            return;
        }
    };
    let token = match gate.turnstile(&login_page(), SITE_KEY).await {
        Ok(token) => token,
        Err(message) => {
            lr(&format!(
                "The turnstile solver failed for account on line {path}"
            ));
            lr(&message);
            return;
        }
    };

    let outcome = crate::ops::auth::login::login(&account, &token).await;
    if !outcome.ok {
        return;
    }
    if crate::ops::auth::login::country_blocked(&outcome.user) {
        lr(&format!(
            "Account on line {path} was refused with Vietnam is blocked by the bot maker"
        ));
        return;
    }

    let entry = match crate::ops::auth::passkey::ensure(&account).await {
        Some(entry) => entry,
        None => {
            lr(&format!(
                "Account on line {path} was skipped without a cached passkey"
            ));
            return;
        }
    };
    let _ = entry;

    let state = match crate::ops::mining::status::read(&account).await {
        Some(state) => state,
        None => return,
    };

    crate::ops::mining::streak::count(&account, state.counted_today, state.streak_days).await;
    lg(&format!(
        "Mining state is {} with {} ZNR available",
        state.state, state.available
    ));
    crate::ops::mining::start::ensure_running(&account, &state.state).await;
    crate::ops::mining::claim::claim_ready(&account, &state.ready_sessions).await;
    crate::ops::task::rewards::claim_all(&account).await;
    crate::ops::referral::wallet::report(&account).await;

    if let Some(closed) = crate::ops::mining::status::read(&account).await {
        lg(&format!(
            "Cycle closed with a balance of {} ZNR claimed",
            closed.total_claimed
        ));
    }
}

pub async fn cycle(accounts: &[AccountLine], solver_url: &str, sleep_seconds: u64) {
    let mut number = 0u64;
    loop {
        number += 1;
        ly(&format!("Starting automation cycle number {number}"));
        for (index, line) in accounts.iter().enumerate() {
            gap(index);
            run_account(line.clone(), solver_url).await;
        }
        crate::support::log::countdown(sleep_seconds, "Next cycle starts in").await;
        crate::app::bootstrap::show_banner();
    }
}
