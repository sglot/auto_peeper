
#[derive(Debug, Clone)]
pub struct ReportAVGPriceByModel {
    pub month: String,
    pub model: String,
    pub avg: f32,
}

impl ReportAVGPriceByModel {
    pub fn new() -> Self {
        Self {
            month: "".to_string(),
            model: "".to_string(),
            avg: 0.0,
        }
    }
}
