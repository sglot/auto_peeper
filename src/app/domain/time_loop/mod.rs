use std::thread::sleep;
use std::time::{Duration, Instant};

use crate::REQUEST_CONFIG;
use crate::app::domain::data_loader;

pub async fn do_loop() {

    let interval = match REQUEST_CONFIG.frequency_unit.as_str() {
        "s" => Duration::from_secs(REQUEST_CONFIG.frequency),
        "m" => Duration::from_secs(REQUEST_CONFIG.frequency * 60),
        "h" => Duration::from_secs(REQUEST_CONFIG.frequency * 60 * 60),
        &_ => Duration::from_secs(REQUEST_CONFIG.frequency),
    }; 

    let mut next_time = Instant::now() + interval;

    loop {
        data_loader::load().await;

        println!("next time {:?}", next_time);

        sleep(next_time - Instant::now());

        next_time += interval;
    }
}
