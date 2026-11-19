#[derive(Clone, Debug)]
pub struct Campaign {
    pub id: String,
    pub title: String,
    pub kind: String,
    pub verification: String,
    pub available: bool,
}

impl Campaign {
    pub fn list(body: &serde_json::Value) -> Vec<Campaign> {
        body.get("campaigns")
            .and_then(|item| item.as_array())
            .map(|items| {
                items
                    .iter()
                    .map(|item| Campaign {
                        id: crate::domain::mining::text(item.get("id")),
                        title: crate::domain::mining::text(item.get("title")),
                        kind: crate::domain::mining::text(item.get("type")),
                        verification: crate::domain::mining::text(item.get("verification")),
                        available: item
                            .get("user")
                            .and_then(|user| user.get("state"))
                            .and_then(|state| state.as_str())
                            == Some("available"),
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn slug_or(&self) -> String {
        if self.title.is_empty() {
            "reward".to_string()
        } else {
            self.title.clone()
        }
    }
}
