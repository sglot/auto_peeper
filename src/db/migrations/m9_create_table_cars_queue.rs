use log::{error, info};
use rusqlite::Connection;

use super::Migration;

pub struct CreateTableCarsQueue {
    name: String,
}

impl CreateTableCarsQueue {
    pub fn new() -> Self {
        Self {
            name: "m9_create_table_cars_queue".to_string(),
        }
    }
}

impl Migration for CreateTableCarsQueue {
    fn up(&self, conn: &Connection) {
        match conn.execute(
            "CREATE TABLE if not exists cars_queue (
                id INTEGER PRIMARY KEY,
                car_id INTEGER NOT NULL,
                chat_id TEXT NOT NULL,
                UNIQUE(car_id, chat_id)
            )",
            (),
        ) {
            Ok(_) => {
                info!("{} migrated", self.name);
                ()
            }
            Err(err) => {error!("Migration {}  error: {}", self.name, err); panic!("")},
        };
    }

    fn down(&self, _conn: &Connection) {}

    fn get_name(&self) -> &str {
        &self.name
    }
}
