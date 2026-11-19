#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProxyEntry(pub String);

impl ProxyEntry {
    pub fn parse(raw: &str) -> Option<ProxyEntry> {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            return None;
        }
        if trimmed.contains("://") {
            Some(ProxyEntry(trimmed.to_string()))
        } else {
            Some(ProxyEntry(format!("http://{trimmed}")))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_the_scheme_when_missing() {
        assert_eq!(
            ProxyEntry::parse("1.2.3.4:8080").unwrap().as_str(),
            "http://1.2.3.4:8080"
        );
    }

    #[test]
    fn keeps_an_existing_scheme() {
        assert_eq!(
            ProxyEntry::parse("socks5://1.2.3.4:1080").unwrap().as_str(),
            "socks5://1.2.3.4:1080"
        );
    }
}
