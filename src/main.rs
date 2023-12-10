use std::sync::Mutex;

use crate::{app::domain::{time_loop, models::context::Context}, config::request_config::RequestConfig};

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

    time_loop::do_loop().await;

}
