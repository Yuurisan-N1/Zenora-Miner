use crate::app::bootstrap::{project_root, show_banner};
use crate::support::cfg::{load_accounts, load_solver_url, Config};
use crate::support::log::lr;

pub async fn run_default() -> i32 {
    show_banner();
    let root = project_root();
    let config = match Config::load(&root) {
        Ok(config) => config,
        Err(message) => {
            lr(&format!("config.json could not be read: {message}"));
            return 1;
        }
    };
    let accounts = load_accounts(&root);
    if accounts.is_empty() {
        lr("File data.txt is empty and holds no email password pairs");
        return 1;
    }
    let solver_url = match load_solver_url(&root) {
        Ok(url) => url,
        Err(message) => {
            lr(&message);
            return 1;
        }
    };
    crate::ops::runner::cycle(&accounts, &solver_url, config.sleep_seconds).await;
    0
}
