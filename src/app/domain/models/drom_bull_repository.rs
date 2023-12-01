use super::drom_bull::DromBull;
use crate::db::connection::sqlite_connection_factory;
use chrono::prelude::*;
use log::{error, info};
use rusqlite::named_params;

#[derive(Default)]
pub struct DromBullRepository {}

impl DromBullRepository {
    pub fn new() -> Self {
        Self {}
    }

    pub fn save(drom_bull: &mut DromBull) {
        let conn = sqlite_connection_factory::get();

        let mut stmt = conn.prepare("INSERT INTO drom_bulls (external_id, firm, model, year, complectation, motor_volume, motor_power, fuel, kpp, privod, probeg, price, location, date, created_at) 
        VALUES (:external_id, :firm, :model, :year, :complectation, :motor_volume, :motor_power, :fuel, :kpp, :privod, :probeg, :price, :location, :date, :created_at)").unwrap();
        let local: DateTime<Local> = Local::now();

        match stmt.execute(named_params! {
            ":external_id": drom_bull.external_id,
            ":firm": drom_bull.firm,
            ":model": drom_bull.model,
            ":year": drom_bull.year,
            ":complectation": drom_bull.complectation,
            ":motor_volume": drom_bull.motor_volume,
            ":motor_power": drom_bull.motor_power,
            ":fuel": drom_bull.fuel,
            ":kpp": drom_bull.kpp,
            ":privod": drom_bull.privod,
            ":probeg": drom_bull.probeg,
            ":price": drom_bull.price,
            ":location": drom_bull.location,
            ":date": local.format("%Y-%d-%m").to_string(),
            ":created_at": local.format("%Y-%m-%d %H:%M:%S").to_string(),
        }) {
            Ok(_) => {
                info!("saved {:?}", drom_bull.external_id);
                ()
            }
            Err(err) => error!("creation error: {}", err),
        };
    }

    pub fn get_identical(external_id: &str, price: u32) -> DromBull {
        let conn = sqlite_connection_factory::get();

        let mut stmt = conn.prepare("SELECT * FROM drom_bulls WHERE external_id = :external_id and price = :price limit 1").unwrap();

        let iter = stmt
            .query_map(
                named_params! {
                    ":external_id": external_id,
                    ":price": price,
                },
                |row| {
                    Ok(DromBull {
                        id: row.get(0)?,
                        external_id: row.get(1)?,
                        firm: row.get(2)?,
                        model: row.get(3)?,
                        year: row.get(4)?,
                        complectation: row.get(5)?,
                        motor_volume: row.get(6)?,
                        motor_power: row.get(7)?,
                        fuel: row.get(8)?,
                        kpp: row.get(9)?,
                        privod: row.get(10)?,
                        probeg: row.get(11)?,
                        price: row.get(12)?,
                        location: row.get(13)?,
                        date: row.get(14)?,
                        created_at: row.get(15)?,
                    })
                },
            )
            .unwrap();

        match iter.last() {
            Some(drom_bull) => 
                match drom_bull {
                    Ok(value) => value,
                    Err(_) => DromBull::new(),
                }
            ,
            None => DromBull::new(),
        }
    }
}
