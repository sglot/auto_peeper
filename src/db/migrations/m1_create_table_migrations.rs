use log::{error, info};
use rusqlite::Connection;

use super::Migration;

pub struct CreateTableMigrations {
    name: String,
}

impl CreateTableMigrations {
    pub fn new() -> Self {
        Self {
            name: "create_table_migrations".to_string(),
        }
    }
}

impl Migration for CreateTableMigrations {
    fn up(&self, conn: &Connection) {
        match conn.execute(
            "CREATE TABLE if not exists migrations (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                created_at TEXT NOT NULL
            )",
            (), // empty list of parameters.
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
