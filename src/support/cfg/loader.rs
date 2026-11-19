use crate::domain::account::AccountLine;
use crate::support::consts::app::{CONFIG_PATH, DATA_PATH, DEFAULTS, PROXY_PATH, SOLVER_YAML};
use std::path::Path;

pub struct Config {
    pub sleep_seconds: u64,
}

impl Config {
    pub fn load(root: &Path) -> std::io::Result<Config> {
        let path = root.join(CONFIG_PATH);
        if !path.exists() {
            std::fs::write(&path, DEFAULTS)?;
        }
        let raw = std::fs::read_to_string(&path)?;
        let parsed: serde_json::Value = serde_json::from_str(&raw)?;
        Ok(Config {
            sleep_seconds: crate::support::cfg::schema::sleep_seconds(&parsed),
        })
    }
}

pub fn load_lines(root: &Path, name: &str) -> Vec<String> {
    let path = root.join(name);
    match std::fs::read_to_string(path) {
        Ok(text) => text
            .lines()
            .map(|line| line.trim().to_string())
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
            .collect(),
        Err(_) => Vec::new(),
    }
}

pub fn load_proxies(root: &Path) -> Vec<String> {
    load_lines(root, PROXY_PATH)
}

pub fn load_accounts(root: &Path) -> Vec<AccountLine> {
    let proxies = load_proxies(root);
    let raw = load_lines(root, DATA_PATH);
    let mut accounts = Vec::new();
    for (index, line) in raw.iter().enumerate() {
        let parts: Vec<&str> = line.splitn(2, '|').collect();
        if parts.len() != 2 {
            continue;
        }
        let proxy = if proxies.is_empty() {
            None
        } else {
            Some(proxies[index % proxies.len()].clone())
        };
        accounts.push(AccountLine::new(
            parts[0].trim(),
            parts[1].trim(),
            proxy,
            index + 1,
        ));
    }
    accounts
}

pub fn load_solver_url(root: &Path) -> Result<String, String> {
    let path = root.join(SOLVER_YAML);
    let text = std::fs::read_to_string(&path)
        .map_err(|_| "solver/solver.yaml was not found".to_string())?;
    let mut in_solver = false;
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if !line.starts_with([' ', '\t']) {
            in_solver = trimmed == "solver:";
            continue;
        }
        if in_solver {
            if let Some(rest) = trimmed.strip_prefix("url:") {
                let url = rest.trim().trim_matches('"').trim_matches('\'');
                if !url.is_empty() {
                    return Ok(url.trim_end_matches('/').to_string());
                }
            }
        }
    }
    Err("solver url is empty: set solver.url in solver/solver.yaml".to_string())
}
