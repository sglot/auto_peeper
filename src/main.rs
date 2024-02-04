use std::sync::Mutex;

use app::domain::queue::queue_loop;

use crate::{app::domain::{time_loop, bot::bot_loop, models::context::Context}, config::request_config::RequestConfig};

pub mod bootstrap;
pub mod app;
pub mod db;
pub mod config;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    #[derive(Debug)]
    pub static ref REQUEST_CONFIG: RequestConfig = RequestConfig::new();
    pub static ref CONTEXT: Mutex<Context> = Mutex::new(Context::new());
}

#[tokio::main]
async fn main() {
    bootstrap::bootstrap::bootstrap::init();

    let app_mode = std::env::var("MODE").expect("MODE must be set");

    match app_mode.as_str() {
        "bot" => bot_loop::do_loop().await,
        "queue" => queue_loop::do_loop().await,
        _ => time_loop::do_loop().await,
        // queue => queue_loop::do_loop().await,
    }
}
