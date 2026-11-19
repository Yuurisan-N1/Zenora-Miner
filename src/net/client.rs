use crate::api::endpoint::BASE_URL;
use crate::net::proxy::ProxyEntry;
use std::time::Duration;

pub struct Reply {
    pub status: u16,
    pub body: serde_json::Value,
}

pub struct Api {
    client: reqwest::Client,
}

impl Api {
    pub fn build(proxy: Option<&ProxyEntry>) -> Result<Api, String> {
        let mut builder = reqwest::Client::builder()
            .cookie_store(true)
            .timeout(Duration::from_secs(30))
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36");
        if let Some(entry) = proxy {
            let parsed = reqwest::Proxy::all(entry.as_str()).map_err(|_| {
                format!(
                    "the proxy {} could not be used; check the host and port",
                    crate::support::log::mask_proxy(entry.as_str())
                )
            })?;
            builder = builder.proxy(parsed);
        }
        let client = builder
            .build()
            .map_err(|_| "the http client could not be created; check proxy.txt".to_string())?;
        Ok(Api { client })
    }

    pub async fn get(&self, path: &str, headers: &[(&str, String)]) -> Reply {
        self.send(reqwest::Method::GET, path, None, headers).await
    }

    pub async fn post(
        &self,
        path: &str,
        body: serde_json::Value,
        headers: &[(&str, String)],
    ) -> Reply {
        self.send(reqwest::Method::POST, path, Some(body), headers)
            .await
    }

    async fn send(
        &self,
        method: reqwest::Method,
        path: &str,
        body: Option<serde_json::Value>,
        headers: &[(&str, String)],
    ) -> Reply {
        let url = format!("{BASE_URL}/api{path}");
        let mut request = self.client.request(method, &url);
        for (name, value) in headers {
            request = request.header(*name, value);
        }
        if let Some(payload) = body {
            request = request.json(&payload);
        }
        match request.send().await {
            Ok(reply) => {
                let status = reply.status().as_u16();
                let body = reply
                    .json::<serde_json::Value>()
                    .await
                    .unwrap_or(serde_json::json!({}));
                Reply {
                    status,
                    body: if body.is_object() {
                        body
                    } else {
                        serde_json::json!({})
                    },
                }
            }
            Err(_) => Reply {
                status: 0,
                body: serde_json::json!({}),
            },
        }
    }
}
