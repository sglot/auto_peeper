#[derive(Debug, Clone)]
pub struct CarsQueue {
    pub id: u64,
    pub car_id: u64,
    pub chat_id: i64,
}

impl CarsQueue {
    pub fn new() -> Self {
        Self {
            id: 0,
            car_id: 0,
            chat_id: 0,
        }
    }
}