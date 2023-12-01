use crate::{app::domain::time_loop, config::request_config::RequestConfig};

pub mod bootstrap;
pub mod app;
pub mod db;
pub mod config;


// use rusqlite::{Connection, OpenFlags, Result};

#[macro_use]
extern crate lazy_static;

lazy_static! {
    #[derive(Debug)]
    pub static ref REQUEST_CONFIG: RequestConfig = RequestConfig::new();
}

#[tokio::main]
async fn main() {
    bootstrap::bootstrap::bootstrap::init();

    time_loop::do_loop().await;

}
