use crate::app::bootstrap::{project_root, show_banner};
use crate::support::cfg::{load_solver_url, Config};
use crate::support::log::{lg, lr, ly};

pub async fn run() -> i32 {
    show_banner();
    let root = project_root();
    let config = match Config::load(&root) {
        Ok(config) => config,
        Err(message) => {
            lr(&format!("config.json could not be read: {message}"));
            return 1;
        }
    };
    lg(&format!(
        "sleep_seconds resolves to {}",
        config.sleep_seconds
    ));
    match load_solver_url(&root) {
        Ok(url) => lg(&format!("solver url resolves to {url}")),
        Err(message) => {
            lr(&message);
            return 1;
        }
    }
    let local = crate::ops::runner::exit_country(None).await;
    if local.is_empty() {
        ly("The exit country could not be read");
    } else {
        lg(&format!("This machine exits through {local}"));
    }
    let url = load_solver_url(&root).unwrap_or_default();
    let mut gate = match crate::net::solver::Gate::new(&url) {
        Ok(gate) => gate,
        Err(message) => {
            lr(&message);
            return 1;
        }
    };
    match gate.open().await {
        Ok(()) => lg("The solver handshake was accepted"),
        Err(message) => {
            lr(&message);
            return 1;
        }
    }
    0
}
