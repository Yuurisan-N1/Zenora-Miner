use crate::net::client::{Api, Reply};
use crate::net::proxy::ProxyEntry;

#[derive(Clone, Debug)]
pub struct AccountLine {
    pub email: String,
    pub password: String,
    pub proxy: Option<String>,
    pub line: usize,
}

impl AccountLine {
    pub fn new(email: &str, password: &str, proxy: Option<String>, line: usize) -> AccountLine {
        AccountLine {
            email: email.to_string(),
            password: password.to_string(),
            proxy,
            line,
        }
    }

    pub fn entry(&self) -> Option<ProxyEntry> {
        self.proxy.as_deref().and_then(ProxyEntry::parse)
    }
}

pub struct Account {
    pub line: AccountLine,
    pub api: Api,
}

impl Account {
    pub fn open(line: AccountLine) -> Result<Account, String> {
        let api = Api::build(line.entry().as_ref())?;
        Ok(Account { line, api })
    }

    pub fn path(&self) -> usize {
        self.line.line
    }

    pub async fn get(&self, path: &str, headers: &[(&str, String)]) -> Reply {
        self.api.get(path, headers).await
    }

    pub async fn post(
        &self,
        path: &str,
        body: serde_json::Value,
        headers: &[(&str, String)],
    ) -> Reply {
        self.api.post(path, body, headers).await
    }
}
