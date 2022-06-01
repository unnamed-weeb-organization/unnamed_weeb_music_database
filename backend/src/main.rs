mod constants;
mod controllers;
mod models;
mod utils;

use tracing::info;
use utils::{config, startup};

#[tokio::main]
async fn main() {
    let conf = config::get_config();
    let (server, addr) = startup::up(conf);

    info!("App is running on: {}", addr);

    if let Err(err) = server.await {
        eprintln!("Server error: {err}");
    }
}
