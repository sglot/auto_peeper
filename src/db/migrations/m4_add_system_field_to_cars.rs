use log::{error, info};
use rusqlite::Connection;

use super::Migration;

pub struct AddSystemFieldToCars {
    name: String,
}

impl AddSystemFieldToCars {
    pub fn new() -> Self {
        Self {
            name: "m4_add_system_field_to_cars".to_string(),
        }
    }
}

impl Migration for AddSystemFieldToCars {
    fn up(&self, conn: &Connection) {
        match conn.execute(
            "ALTER TABLE cars ADD system VARCHAR;",
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
