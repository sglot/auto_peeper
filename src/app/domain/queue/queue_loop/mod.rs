use std::thread::sleep;
use std::time::{Duration, Instant};

use crate::app::domain::models::car_repository::CarRepository;
use crate::app::domain::models::cars_queue_repository::CarsQueueRepository;
use crate::app::domain::models::context_repository::ContextRepository;
use crate::REQUEST_CONFIG;

use teloxide::{prelude::*, utils::command::BotCommands};

pub async fn do_loop() {
    log::info!("Starting queue...");

    let interval = match REQUEST_CONFIG.frequency_unit.as_str() {
        "s" => Duration::from_secs(REQUEST_CONFIG.frequency),
        "m" => Duration::from_secs(REQUEST_CONFIG.frequency * 60),
        "h" => Duration::from_secs(REQUEST_CONFIG.frequency * 60 * 60),
        &_ => Duration::from_secs(REQUEST_CONFIG.frequency),
    };

    let mut next_time = Instant::now() + interval;

    ContextRepository::set_true();

    loop {
        let queue_rows = CarsQueueRepository::all();
        let bot = Bot::from_env();
        log::info!("{:?}", queue_rows);

        if queue_rows.len() == 0 {
            sleep(next_time - Instant::now());

            next_time += interval;

            continue;
        }

        let mut chat_id = queue_rows.first().unwrap().chat_id;
        let mut message = "".to_string();
        let mut count = 1;
        let mut forDelete = Vec::new();
        for queue in queue_rows {
            if !chat_id.eq(&queue.chat_id) {
                break;
            }
            chat_id = queue.chat_id;

            let car = CarRepository::get_by_id(queue.car_id);

            message.push_str(
                format!("{:?} {:?} \n {:?} \n\n", car.firm, car.model, car.link).as_str(),
            );

            forDelete.push(queue.id);
            
            log::info!("{:?}", car);

            count += 1;

            if count > 10 {
                break;
            }
        }

        if !message.is_empty() {
            bot.send_message(ChatId(chat_id), message.clone())
                .await
                .unwrap();

            for forDel in forDelete {
                CarsQueueRepository::delete(forDel);
            }
        }

        log::info!("{:?}", message);
        println!("next time {:?}", next_time);

        sleep(next_time - Instant::now());

        next_time += interval;
    }
}
