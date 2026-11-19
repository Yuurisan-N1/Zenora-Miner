use super::init::{emit, live, G, R, Y};
use std::io::IsTerminal;
use std::time::Duration;

pub fn lg(msg: &str) {
    emit(G, msg);
}

pub fn ly(msg: &str) {
    emit(Y, msg);
}

pub fn lr(msg: &str) {
    emit(R, msg);
}

pub fn blank() {
    super::init::hard_blank();
}

pub fn gap(index: usize) {
    if index > 0 {
        blank();
    }
}

pub fn status(msg: &str) {
    live(Y, msg);
}

pub fn finish_status() {
    super::init::clear_live();
}

pub fn format_duration(seconds: u64) -> String {
    let days = seconds / 86_400;
    let rest = seconds % 86_400;
    let hours = rest / 3_600;
    let minutes = (rest % 3_600) / 60;
    let secs = rest % 60;
    if days > 0 {
        format!("{days:02}:{hours:02}:{minutes:02}:{secs:02}")
    } else {
        format!("{hours:02}:{minutes:02}:{secs:02}")
    }
}

pub async fn countdown(seconds: u64, label: &str) {
    if seconds < 1 {
        return;
    }
    if !std::io::stdout().is_terminal() {
        status(&format!("{label} {}", format_duration(seconds)));
        finish_status();
        tokio::time::sleep(Duration::from_secs(seconds)).await;
        return;
    }
    let mut left = seconds;
    while left > 0 {
        if crate::support::log::stopped() {
            return;
        }
        status(&format!("{label} {}", format_duration(left)));
        tokio::time::sleep(Duration::from_secs(1)).await;
        left -= 1;
    }
    finish_status();
}

pub fn mask_email(value: &str) -> String {
    match value.split_once('@') {
        Some((head, domain)) => {
            let taken: String = head.chars().take(2).collect();
            let stars = "*".repeat(head.chars().count().saturating_sub(2).max(3));
            format!("{taken}{stars}@{domain}")
        }
        None => "***".to_string(),
    }
}

pub fn mask_proxy(proxy_url: &str) -> String {
    let after_scheme = proxy_url.rsplit("://").next().unwrap_or(proxy_url);
    let after_at = after_scheme.rsplit('@').next().unwrap_or(after_scheme);
    let mut parts = after_at.splitn(2, ':');
    let host = parts.next().unwrap_or("");
    let port = parts.next().unwrap_or("");
    let masked_host = if host.split('.').count() == 4 {
        let octets: Vec<&str> = host.split('.').collect();
        format!("{}*****{}", octets[0], octets[3])
    } else if host.chars().count() > 4 {
        let chars: Vec<char> = host.chars().collect();
        let head: String = chars.iter().take(2).collect();
        let tail: String = chars[chars.len() - 2..].iter().collect();
        format!("{head}*****{tail}")
    } else {
        "***".to_string()
    };
    if port.is_empty() {
        format!("http://user:pass@{masked_host}")
    } else {
        format!("http://user:pass@{masked_host}:{port}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mask_email_counts_chars() {
        assert_eq!(mask_email("lamawan74@gmail.com"), "la*******@gmail.com");
        assert_eq!(mask_email("nope"), "***");
    }

    #[test]
    fn mask_proxy_hides_the_host() {
        assert_eq!(
            mask_proxy("http://user:pass@gw.dataimpulse.com:823"),
            "http://user:pass@gw*****om:823"
        );
    }

    #[test]
    fn duration_switches_shape_after_a_day() {
        assert_eq!(format_duration(59), "00:00:59");
        assert_eq!(format_duration(90_061), "01:01:01:01");
    }
}
