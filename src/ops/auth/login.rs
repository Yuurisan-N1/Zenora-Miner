use crate::api::endpoint::{referer_dashboard, referer_security, LOGIN, ORIGIN};
use crate::domain::account::Account;
use crate::support::log::{lg, lr};

pub const ORIGIN_NAME: &str = "Origin";
pub const REFERER_NAME: &str = "Referer";

pub struct LoginOutcome {
    pub ok: bool,
    pub user: serde_json::Value,
}

pub async fn login(account: &Account, turnstile_token: &str) -> LoginOutcome {
    let body = serde_json::json!({
        "email": account.line.email,
        "password": account.line.password,
        "turnstileToken": turnstile_token,
    });
    let reply = account.post(LOGIN, body, &[]).await;
    match reply.status {
        400 | 401 => {
            lr(&format!(
                "Account on line {} holds invalid login details and was skipped",
                account.path()
            ));
            LoginOutcome {
                ok: false,
                user: serde_json::json!({}),
            }
        }
        200 => {
            if reply
                .body
                .get("mfaRequired")
                .and_then(|v| v.as_bool())
                .unwrap_or(false)
            {
                lr(&format!(
                    "Account on line {} requires two factor login and was skipped",
                    account.path()
                ));
                return LoginOutcome {
                    ok: false,
                    user: serde_json::json!({}),
                };
            }
            let user = reply
                .body
                .get("user")
                .cloned()
                .unwrap_or(serde_json::json!({}));
            let raw = user.get("firstName").and_then(|v| v.as_str()).unwrap_or("");
            let name = crate::app::bootstrap::clean_text(raw, &account.line.email);
            lg(&format!("Signed in {name} on line {}", account.path()));
            LoginOutcome { ok: true, user }
        }
        _ => {
            let code = reply
                .body
                .get("error")
                .or_else(|| reply.body.get("code"))
                .and_then(|value| value.as_str())
                .unwrap_or("");
            if code.is_empty() {
                lr("The login attempt was refused by the server");
            } else {
                lr(&format!(
                    "The login attempt was refused with {}",
                    crate::support::error::refusal(code)
                ));
            }
            LoginOutcome {
                ok: false,
                user: serde_json::json!({}),
            }
        }
    }
}

pub fn security_headers() -> Vec<(&'static str, String)> {
    vec![
        (ORIGIN_NAME, ORIGIN.to_string()),
        (REFERER_NAME, referer_security()),
    ]
}

pub fn dashboard_headers() -> Vec<(&'static str, String)> {
    vec![
        (ORIGIN_NAME, ORIGIN.to_string()),
        (REFERER_NAME, referer_dashboard()),
    ]
}

pub fn country_blocked(user: &serde_json::Value) -> bool {
    let raw = user
        .get("country")
        .or_else(|| user.get("countryCode"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_uppercase();
    let country = crate::app::bootstrap::clean_text(&raw, "");
    country == "VN" || country == "VIETNAM"
}
