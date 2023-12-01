use chrono::{DateTime, Local};
use log::error;
use rusqlite::{named_params, Connection};

pub fn write_migration(name: &str, conn: &Connection) {
    let mut stmt = conn.prepare("INSERT OR IGNORE INTO migrations (name, created_at) VALUES (:name, :created_at)").unwrap();

        let local: DateTime<Local> = Local::now();

        match stmt.execute(named_params! {
            ":name": name,
            ":created_at": local.format("%Y-%m-%d %H:%M:%S").to_string(),
        }) {
            Ok(_) => (),
            Err(err) => {error!("Migration {}  error: {}", name, err); panic!("")},
        };
}