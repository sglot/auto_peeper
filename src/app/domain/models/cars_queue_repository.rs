use crate::db::connection::sqlite_connection_factory;
use log::{error, info};
use rusqlite::named_params;

use super::cars_queue::CarsQueue;

#[derive(Default)]
pub struct CarsQueueRepository {}

impl CarsQueueRepository {
    pub fn new() -> Self {
        Self {}
    }

    pub fn upsert(queue: &mut CarsQueue) {
        let conn = sqlite_connection_factory::get();

        let mut stmt = conn
            .prepare(
                "INSERT INTO cars_queue (car_id, chat_id) 
        VALUES (:car_id, :chat_id) 
        ON CONFLICT(car_id, chat_id) DO UPDATE SET
        car_id = :car_id,  chat_id = :chat_id
        ",
            )
            .unwrap();

        match stmt.execute(named_params! {
            ":car_id": queue.car_id,
            ":chat_id": queue.chat_id,
        }) {
            Ok(_) => {
                info!("cars_queue saved {:?}", queue);
                ()
            }
            Err(err) => error!("creation error: {}", err),
        };
    }

    pub fn all() -> Vec<CarsQueue> {
        let conn = sqlite_connection_factory::get();

        let mut stmt = conn
            .prepare("SELECT * FROM cars_queue ORDER BY chat_id DESC;")
            .unwrap();

        let iter = stmt
            .query_map((), |row| {
                let chat_id: String = row.get(2)?;

                Ok(CarsQueue {
                    id: row.get(0)?,
                    car_id: row.get(1)?,
                    chat_id: chat_id.parse::<i64>().unwrap(),
                })
            })
            .unwrap();

        let mut v: Vec<CarsQueue> = Vec::new();

        for res in iter {
            match res {
                Ok(p) => {
                    v.push(p);
                    ()
                }
                Err(_) => (),
            };
        }

        v
    }

    pub fn delete(id: u64) -> () {
        let conn = sqlite_connection_factory::get();

        let mut stmt = conn
            .prepare("DELETE FROM cars_queue WHERE id = :id")
            .unwrap();

        stmt.execute(named_params! {
            ":id": id,
        })
        .unwrap();

        info!("{} cars_queue removed", id);
    }
}
