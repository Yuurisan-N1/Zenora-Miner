#[derive(Clone, Debug, Default)]
pub struct MiningState {
    pub state: String,
    pub counted_today: bool,
    pub streak_days: i64,
    pub available: f64,
    pub total_claimed: f64,
    pub ready_sessions: Vec<String>,
}

impl MiningState {
    pub fn parse(body: &serde_json::Value) -> MiningState {
        let streak = body.get("streak").cloned().unwrap_or(serde_json::json!({}));
        let balance = body
            .get("balance")
            .cloned()
            .unwrap_or(serde_json::json!({}));
        let ready = body
            .get("ready")
            .and_then(|item| item.get("sessions"))
            .and_then(|item| item.as_array())
            .cloned()
            .unwrap_or_default();
        MiningState {
            state: text(body.get("state")),
            counted_today: streak
                .get("countedToday")
                .and_then(|v| v.as_bool())
                .unwrap_or(false),
            streak_days: streak.get("days").and_then(|v| v.as_i64()).unwrap_or(0),
            available: number(balance.get("available")),
            total_claimed: number(balance.get("totalClaimed")),
            ready_sessions: ready
                .iter()
                .map(|item| match item {
                    serde_json::Value::String(value) => value.clone(),
                    other => text(other.get("id")),
                })
                .filter(|value| !value.is_empty())
                .collect(),
        }
    }
}

pub fn text(value: Option<&serde_json::Value>) -> String {
    value
        .and_then(|item| item.as_str())
        .unwrap_or("")
        .to_string()
}

pub fn number(value: Option<&serde_json::Value>) -> f64 {
    match value {
        Some(serde_json::Value::Number(raw)) => raw.as_f64().unwrap_or(0.0),
        Some(serde_json::Value::String(raw)) => raw.parse().unwrap_or(0.0),
        _ => 0.0,
    }
}
