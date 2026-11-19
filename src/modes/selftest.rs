use crate::app::bootstrap::clean_text;
use crate::domain::session::{vault_decrypt, vault_encrypt};
use crate::ops::auth::passkey::{
    b64d, b64e, new_signing_key, pem_decode, pem_encode, sign_assertion,
};
use crate::support::cfg::schema::DEFAULTS;
use crate::support::log::{format_duration, lr, mask_email, mask_proxy};

pub fn run() -> i32 {
    let mut checks = 0usize;
    let mut failed = 0usize;

    macro_rules! check {
        ($label:expr, $condition:expr) => {{
            checks += 1;
            if $condition {
                println!("ok   {}", $label);
            } else {
                failed += 1;
                lr(&format!("FAIL {}", $label));
            }
        }};
    }

    check!("defaults keep the 3600 second sleep", {
        let parsed: serde_json::Value = serde_json::from_str(DEFAULTS).unwrap_or_default();
        crate::support::cfg::schema::sleep_seconds(&parsed) == 3600
    });
    check!(
        "mask_email keeps two characters",
        mask_email("lamawan74@gmail.com") == "la*******@gmail.com"
    );
    check!(
        "mask_proxy hides the host",
        mask_proxy("http://user:pass@gw.dataimpulse.com:823") == "http://user:pass@gw*****om:823"
    );
    check!(
        "countdown formats below one day",
        format_duration(90) == "00:01:30"
    );
    check!(
        "countdown formats above one day",
        format_duration(90_061) == "01:01:01:01"
    );
    check!(
        "clean_text strips banned symbols",
        clean_text("a|b#c", "x") == "a b c"
    );

    let sealed = vault_encrypt("hello-passkey");
    check!(
        "the passkey vault round trips",
        sealed.starts_with("zmvault:")
            && vault_decrypt(&sealed).ok().as_deref() == Some("hello-passkey")
    );

    let key = new_signing_key();
    let pem = pem_encode(&key);
    check!("the passkey pem round trips", pem_decode(&pem).is_ok());

    let assertion = sign_assertion(&key, "challenge", "credential", "zenoranetwork.com", 1);
    let auth_data = assertion["response"]["authenticatorData"]
        .as_str()
        .and_then(|value| b64d(value).ok())
        .unwrap_or_default();
    check!(
        "the assertion carries the sign counter",
        auth_data.len() == 37
    );
    check!(
        "the assertion carries a signature",
        assertion["response"]["signature"]
            .as_str()
            .map(|value| !value.is_empty())
            .unwrap_or(false)
    );
    check!(
        "base64url drops the padding",
        !b64e(b"any carnal pleasure").contains('=')
    );

    let proxy = crate::net::proxy::ProxyEntry::parse("1.2.3.4:8080");
    check!(
        "a bare proxy gains its scheme",
        proxy.as_ref().map(|item| item.as_str()) == Some("http://1.2.3.4:8080")
    );

    println!();
    println!("{checks} checks ran with {failed} failures");
    if failed == 0 {
        0
    } else {
        1
    }
}
