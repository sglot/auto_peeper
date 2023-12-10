use log::{error, info};
use rusqlite::Connection;

use super::Migration;

pub struct RenameTableDromBullsToCars {
    name: String,
}

impl RenameTableDromBullsToCars {
    pub fn new() -> Self {
        Self {
            name: "m3_rename_table_drom_bulls_to_cars".to_string(),
        }
    }
}

impl Migration for RenameTableDromBullsToCars {
    fn up(&self, conn: &Connection) {
        match conn.execute(
            "ALTER TABLE drom_bulls RENAME TO cars",
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
