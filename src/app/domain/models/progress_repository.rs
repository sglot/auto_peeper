use std::collections::HashMap;

use crate::db::connection::sqlite_connection_factory;
use chrono::prelude::*;
use log::{error, info};
use rusqlite::named_params;

use super::progress::Progress;

#[derive(Default)]
pub struct ProgressRepository {}

impl ProgressRepository {
    pub fn new() -> Self {
        Self {}
    }

    pub fn upsert(progress: &mut Progress) {
        let conn = sqlite_connection_factory::get();

        let mut stmt = conn
            .prepare(
                "INSERT INTO progress (request_id, system, page, is_loaded, created_at, updated_at) 
        VALUES (:request_id, :system, :page, :is_loaded, :created_at, :updated_at) 
        ON CONFLICT(request_id, system) DO UPDATE SET
        page = :page, is_loaded = :is_loaded, updated_at = :updated_at
        ",
            )
            .unwrap();
        let local: DateTime<Local> = Local::now();

        match stmt.execute(named_params! {
            ":request_id": progress.request_id,
            ":system": progress.system,
            ":page": progress.page,
            ":is_loaded": progress.is_loaded,
            ":created_at": local.format("%Y-%m-%d %H:%M:%S").to_string(),
            ":updated_at": local.format("%Y-%m-%d %H:%M:%S").to_string(),
        }) {
            Ok(_) => {
                info!("progress saved {:?}", progress.request_id);
                ()
            }
            Err(err) => error!("creation error: {}", err),
        };
    }

    pub fn save(progress: &mut Progress) {
        let conn = sqlite_connection_factory::get();

        let mut stmt = conn
            .prepare(
                "INSERT INTO progress (request_id, system, page, is_loaded, created_at) 
        VALUES (:request_id, :system, :page, :is_loaded, :created_at)",
            )
            .unwrap();
        let local: DateTime<Local> = Local::now();

        match stmt.execute(named_params! {
            ":request_id": progress.request_id,
            ":system": progress.system,
            ":page": progress.page,
            ":is_loaded": progress.is_loaded,
            ":created_at": local.format("%Y-%m-%d %H:%M:%S").to_string(),
            ":updated_at": local.format("%Y-%m-%d %H:%M:%S").to_string(),
        }) {
            Ok(_) => {
                info!("progress saved {:?}", progress.request_id);
                ()
            }
            Err(err) => error!("creation error: {}", err),
        };
    }

    pub fn get_identical(request_id: &str, system: &str) -> Progress {
        let conn = sqlite_connection_factory::get();

        let mut stmt = conn.prepare("SELECT * FROM progress WHERE system = :system and request_id = :request_id limit 1").unwrap();

        let iter = stmt
            .query_map(
                named_params! {
                    ":system": system,
                    ":request_id": request_id,
                },
                |row| {
                    Ok(Progress {
                        id: row.get(0)?,
                        request_id: row.get(1)?,
                        system: row.get(2)?,
                        page: row.get(3)?,
                        is_loaded: row.get(4)?,
                        created_at: row.get(5)?,
                        updated_at: row.get(6)?,
                    })
                },
            )
            .unwrap();

        match iter.last() {
            Some(drom_bull) => match drom_bull {
                Ok(value) => value,
                Err(_) => Progress::new(),
            },
            None => Progress::new(),
        }
    }

    pub fn delete_finished(pack_count: usize, system: &str) -> () {
        let conn = sqlite_connection_factory::get();

        let mut stmt = conn.prepare("SELECT id FROM progress WHERE system = :system ").unwrap();

        let res = stmt.query_map(named_params! {":system": system,}, |row| {
            let mut p = Progress::new();
            info!("row row {:?} ", row);
            p.id = row.get(0)?;
            Ok(p)
        })
        .unwrap();

        let count = res.count();
        if count < pack_count {
            info!("{} progress stay {} < {}", system, count, pack_count);
            return;
        }

        let mut stmt = conn
            .prepare("DELETE FROM progress WHERE system = :system")
            .unwrap();

        stmt.execute(named_params! {
            ":system": system,
        })
        .unwrap();

        info!("{} progress cleared", system);

    }

    

    pub fn all() -> HashMap<String, Progress> {
        let conn = sqlite_connection_factory::get();

        let mut stmt = conn.prepare("SELECT * FROM progress").unwrap();

        let iter = stmt
            .query_map((), |row| {
                Ok(Progress {
                    id: row.get(0)?,
                    request_id: row.get(1)?,
                    system: row.get(2)?,
                    page: row.get(3)?,
                    is_loaded: row.get(4)?,
                    created_at: row.get(5)?,
                    updated_at: row.get(6)?,
                })
            })
            .unwrap();

        let mut map: HashMap<String, Progress> = HashMap::new();

        for res in iter {
            match res {
                Ok(p) => {
                    map.insert(p.key(), p);
                    ()
                }
                Err(_) => (),
            };
        }

        map
    }
}