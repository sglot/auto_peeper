use std::collections::HashMap;

use crate::db::connection::sqlite_connection_factory;
use rusqlite::named_params;

use super::report_avg_price_by_model::ReportAVGPriceByModel;


#[derive(Default)]
pub struct ReportAVGPriceByModelRepository {}

impl ReportAVGPriceByModelRepository {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get(model: &str) -> Vec<ReportAVGPriceByModel> {
        let conn = sqlite_connection_factory::get();

        let mut stmt = conn.prepare("SELECT STRFTIME('%Y.%m', created_at) AS month, model, avg(price) FROM cars WHERE LOWER(model) = :model 
        GROUP BY STRFTIME('%Y.%m', created_at)
        ORDER BY month DESC;").unwrap();

        let iter = stmt
            .query_map(named_params! {
                ":model": model.to_lowercase(),
            }, |row| {
                Ok(ReportAVGPriceByModel {
                    month: row.get(0)?,
                    model: row.get(1)?,
                    avg: row.get(2)?,
                })
            })
            .unwrap();

        let mut v: Vec<ReportAVGPriceByModel> = Vec::new();

        for res in iter {
            match res {
                Ok(p) => {
                    v.push(p);
                    ()
                }
                Err(_) => (),
            };
        }

        v
    }
}
