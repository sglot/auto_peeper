use log::{error, info};
use rusqlite::Connection;

use super::Migration;

pub struct AddExclusiveFieldToCars {
    name: String,
}

impl AddExclusiveFieldToCars {
    pub fn new() -> Self {
        Self {
            name: "m5_add_exclusive_field_to_cars".to_string(),
        }
    }
}

impl Migration for AddExclusiveFieldToCars {
    fn up(&self, conn: &Connection) {
        match conn.execute(
            "ALTER TABLE cars ADD exclusive BOOLEAN DEFAULT false;",
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
