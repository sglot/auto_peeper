use log::{error, info};
use rusqlite::Connection;

use super::Migration;

pub struct CreateTableDromBulls {
    name: String,
}

impl CreateTableDromBulls {
    pub fn new() -> Self {
        Self {
            name: "create_table_drom_bulls".to_string(),
        }
    }
}

impl Migration for CreateTableDromBulls {
    fn up(&self, conn: &Connection) {
        match conn.execute(
            "CREATE TABLE if not exists drom_bulls (
                id INTEGER PRIMARY KEY,
                external_id TEXT,
                firm TEXT NOT NULL,
                model TEXT NOT NULL,
                year NUMERIC NOT NULL,
                complectation  TEXT NOT NULL,
                motor_volume NUMERIC NOT NULL,
                motor_power NUMERIC NOT NULL,
                fuel TEXT NOT NULL,
                kpp TEXT NOT NULL,
                privod TEXT NOT NULL,
                probeg NUMERIC NOT NULL,
                price NUMERIC NOT NULL,
                location TEXT NOT NULL,
                date TEXT NOT NULL,
                created_at TEXT NOT NULL
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
