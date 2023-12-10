use rusqlite::{named_params, Connection};

pub fn check_migration(name: &str, conn: &Connection) -> bool {
    let mut stmt = conn.prepare("SELECT * FROM migrations WHERE name = :name ").unwrap();

        let res = stmt.exists(named_params! {":name": name,});

        match res {
            Ok(exist) => !exist,
            Err(_) => false,
        }
}