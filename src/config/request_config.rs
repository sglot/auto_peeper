#[derive(Debug)]
pub struct RequestConfig {
    pub frequency: u64,
    pub frequency_unit: String,
    pub location: String,
    pub location_file_path: String,
    pub avito_sleep_seconds: u64,
}

impl RequestConfig {
    pub fn new() -> RequestConfig {
        RequestConfig {
            frequency: std::env::var("REQUEST_FREQUENCY").expect("REQUEST_FREQUENCY must be set").parse::<u64>().unwrap_or(60),
            frequency_unit: std::env::var("REQUEST_FREQUENCY_UNIT").expect("REQUEST_FREQUENCY_UNIT must be set"),
            location: std::env::var("REQUEST_LOCATION").expect("REQUEST_LOCATION must be set"),
            location_file_path: std::env::var("REQUEST_LOCATION_FILE_PATH").expect("REQUEST_LOCATION_FILE_PATH must be set"),
            avito_sleep_seconds: std::env::var("REQUEST_AVITO_SLEEP_SECONDS").expect("REQUEST_AVITO_SLEEP_SECONDS must be set").parse::<u64>().unwrap_or(120),
        }
    }
}
