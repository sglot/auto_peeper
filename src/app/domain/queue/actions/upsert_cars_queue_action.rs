use crate::app::domain::models::{
    cars_queue::CarsQueue, cars_queue_repository::CarsQueueRepository, notification_member_repository::NotificationMemberRepository,
};

pub fn upsert_cars_queue(car_id: i64, car_price: u32) {
    let members = NotificationMemberRepository::search(1500000);
    log::info!("car_price = {:?},  {:?}", car_price, members);

    for member in members {
        let mut cars_queue = CarsQueue::new();
        cars_queue.car_id = u64::try_from(car_id).unwrap();
        cars_queue.chat_id = member.chat_id;

        CarsQueueRepository::upsert(&mut cars_queue);
    }
}
