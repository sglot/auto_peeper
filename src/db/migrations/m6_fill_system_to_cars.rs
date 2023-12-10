use log::{error, info};
use rusqlite::Connection;

use super::Migration;

pub struct FillSystemToCars {
    name: String,
}

impl FillSystemToCars {
    pub fn new() -> Self {
        Self {
            name: "m6_fill_system_to_cars".to_string(),
        }
    }
}

impl Migration for FillSystemToCars {
    fn up(&self, conn: &Connection) {
        match conn.execute(
            "UPDATE cars SET system = 'drom';",
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
