// машина на дроме
// TODO переименовать
#[derive(Default, Debug)]
pub struct DromBull {
    pub id: u64,
    pub external_id: String,
    pub firm: String,
    pub model: String,
    pub year: u32,
    pub complectation: String,
    pub motor_volume: f32,
    pub motor_power: u32,
    pub fuel: String,
    pub kpp: String,
    pub privod: String,
    pub probeg: u32,
    pub price: u32,
    pub location: String,

    pub system: String,
    pub exclusive: bool,

    pub date: String,
    pub created_at: String,
}

impl DromBull {
    pub fn new() -> Self {
        Self {
            id: 0,
            external_id: "".to_string(),
            firm: "".to_string(),
            model: "".to_string(),
            year: 2023,
            complectation: "".to_string(),
            motor_volume: 0.0,
            motor_power: 0,
            fuel: "".to_string(),
            kpp: "".to_string(),
            privod: "".to_string(),
            probeg: 0,
            price: 0,
            location: "".to_string(),

            system: "".to_string(),
            exclusive: false,

            date: "".to_string(),
            created_at : "".to_string(),
        }
    }
}
