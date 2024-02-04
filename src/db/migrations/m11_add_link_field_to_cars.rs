use log::{error, info};
use rusqlite::Connection;

use super::Migration;

pub struct AddLinkFieldToCars {
    name: String,
}

impl AddLinkFieldToCars {
    pub fn new() -> Self {
        Self {
            name: "m11_add_link_field_to_cars".to_string(),
        }
    }
}

impl Migration for AddLinkFieldToCars {
    fn up(&self, conn: &Connection) {
        match conn.execute(
            "ALTER TABLE cars ADD link TEXT;",
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
