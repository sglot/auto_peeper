use std::thread::sleep;
use std::time::{Duration, Instant};

use log::info;

use crate::app::domain::models::context_repository::ContextRepository;
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
    
    ContextRepository::set_true();

    loop {

        // запасная проверка на перехлёст таймаута основного и в работе какого-нибудь из клиентов
        if !ContextRepository::next_round() {
            info!("\n lock wait next_round \n");
            sleep(Duration::from_secs(1));
            continue;
        }

        ContextRepository::set_default();

        data_loader::load().await;

        // запасная проверка на перехлёст таймаута основного и в работе какого-нибудь из клиентов
        if !ContextRepository::next_round() {
            info!("\n lock wait next_round \n");
            sleep(Duration::from_secs(1));
            continue;
        }
        println!("next time {:?}", next_time);

        sleep(next_time - Instant::now());

        next_time += interval;
    }
}
