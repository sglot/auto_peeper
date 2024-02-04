use log::{error, info};
use rusqlite::Connection;

use super::Migration;

pub struct AddNewFieldToCars {
    name: String,
}

impl AddNewFieldToCars {
    pub fn new() -> Self {
        Self {
            name: "m8_add_new_field_to_cars".to_string(),
        }
    }
}

impl Migration for AddNewFieldToCars {
    fn up(&self, conn: &Connection) {
        match conn.execute(
            "ALTER TABLE cars ADD new BOOLEAN DEFAULT false;",
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
