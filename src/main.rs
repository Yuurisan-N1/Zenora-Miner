mod api;
mod app;
mod domain;
mod modes;
mod net;
mod ops;
mod support;

#[tokio::main]
async fn main() {
    support::log::arm();
    tokio::spawn(async {
        if tokio::signal::ctrl_c().await.is_ok() {
            support::log::stop_line("Script stopped by user");
            std::process::exit(0);
        }
    });

    let mode = std::env::args().nth(1).unwrap_or_else(|| "run".to_string());
    let exit = match mode.as_str() {
        "run" => modes::run::run_default().await,
        "probe" => modes::probe::run().await,
        "selftest" => modes::selftest::run(),
        _ => {
            support::log::lr("Unknown mode; use run, probe or selftest");
            1
        }
    };
    std::process::exit(exit);
}
