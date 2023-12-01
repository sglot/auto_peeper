use self::drom_client::DromClient;
use super::models::user_request::UserRequest;
use async_trait::async_trait;

pub mod drom_client;

pub fn get_client() -> impl PeeperClient {
    DromClient::new()
}

#[async_trait]
pub trait PeeperClient {
    async fn search(&mut self, request: &UserRequest);
}
