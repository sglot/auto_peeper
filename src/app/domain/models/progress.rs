use crate::app::domain::support;

#[derive(Debug, Clone)]
pub struct Progress {
    pub id: u64,
    pub request_id: u32,
    pub system: String,
    pub page: u32,
    pub is_loaded: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl Progress {
    pub fn new() -> Self {
        Self {
            id: 0,
            request_id: 0,
            system: "".to_string(),
            page: 0,
            is_loaded: false,
            created_at: "".to_string(),
            updated_at: "".to_string(),
        }
    }

    pub fn key(&self) -> String {
        support::make_key(self.request_id.to_string(), self.system.as_str())
    }
}