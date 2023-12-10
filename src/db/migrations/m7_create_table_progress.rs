use log::{error, info};
use rusqlite::Connection;

use super::Migration;

pub struct CreateTableProgress {
    name: String,
}

impl CreateTableProgress {
    pub fn new() -> Self {
        Self {
            name: "m7_create_table_progress".to_string(),
        }
    }
}

impl Migration for CreateTableProgress {
    fn up(&self, conn: &Connection) {
        match conn.execute(
            "CREATE TABLE if not exists progress (
                id INTEGER PRIMARY KEY,
                request_id INTEGER NOT NULL,
                system TEXT NOT NULL,
                page INTEGER NOT NULL,
                is_loaded BOOLEAN NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                UNIQUE(request_id, system)
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
