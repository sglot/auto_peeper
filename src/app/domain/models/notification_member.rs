#[derive(Debug, Clone)]
pub struct NotificationMember {
    pub id: u64,
    pub min_price: u32,
    pub max_price: u32,
    pub chat_id: i64,
}

impl NotificationMember {
    pub fn new() -> Self {
        Self {
            id: 0,
            min_price: 0,
            max_price: 0,
            chat_id: 0,
        }
    }
}