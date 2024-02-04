use self::{drom_client::DromClient, peeper_client_enum::PeeperClientType, avito_client::AvitoClient};
use super::models::user_request::UserRequest;
use async_trait::async_trait;

pub mod drom_client;
pub mod avito_client;
pub mod peeper_client_enum;


pub fn get_client(client_type: PeeperClientType) -> Box<dyn PeeperClient> {
    match client_type {
        PeeperClientType::Drom => Box::new(DromClient::new()),
        PeeperClientType::Avito => Box::new(AvitoClient::new()),
    }
    
}

#[async_trait]
pub trait PeeperClient: Send {
    async fn search(&mut self, request: &UserRequest, start_page: &u32);
}
