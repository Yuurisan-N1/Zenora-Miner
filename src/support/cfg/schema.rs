pub const DEFAULTS: &str = "{\n  \"settings\": {\n    \"sleep_seconds\": 3600\n  }\n}\n";

pub fn sleep_seconds(value: &serde_json::Value) -> u64 {
    value
        .pointer("/settings/sleep_seconds")
        .and_then(|item| item.as_u64())
        .map(|seconds| seconds.clamp(30, 86_400))
        .unwrap_or(3600)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_match_the_canon_block() {
        assert_eq!(DEFAULTS.len(), 50);
        assert_eq!(DEFAULTS.lines().count(), 5);
        let parsed: serde_json::Value = serde_json::from_str(DEFAULTS).unwrap();
        assert_eq!(sleep_seconds(&parsed), 3600);
    }
}
