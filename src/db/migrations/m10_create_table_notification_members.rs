use log::{error, info};
use rusqlite::Connection;

use super::Migration;

pub struct CreateTableNotificationMembers {
    name: String,
}

impl CreateTableNotificationMembers {
    pub fn new() -> Self {
        Self {
            name: "m10_create_table_notification_members".to_string(),
        }
    }
}

impl Migration for CreateTableNotificationMembers {
    fn up(&self, conn: &Connection) {
        match conn.execute(
            "CREATE TABLE if not exists notification_members (
                id INTEGER PRIMARY KEY,
                chat_id TEXT NOT NULL,
                min_price NUMERIC NOT NULL,
                max_price NUMERIC NOT NULL,
                UNIQUE(chat_id)
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
