use crate::db::connection::sqlite_connection_factory;
use log::{error, info};
use rusqlite::named_params;

use super::notification_member::NotificationMember;

#[derive(Default)]
pub struct NotificationMemberRepository {}

impl NotificationMemberRepository {
    pub fn new() -> Self {
        Self {}
    }

    pub fn upsert(member: &mut NotificationMember) {
        let conn = sqlite_connection_factory::get();

        let mut stmt = conn
            .prepare(
                "INSERT INTO notification_members (min_price, max_price, chat_id) 
        VALUES (:min_price, :max_price, :chat_id) 
        ON CONFLICT(chat_id) DO UPDATE SET
        min_price = :min_price, max_price = :max_price,  chat_id = :chat_id
        ",
            )
            .unwrap();

        match stmt.execute(named_params! {
            ":min_price": member.min_price,
            ":max_price": member.max_price,
            ":chat_id": member.chat_id,
        }) {
            Ok(_) => {
                info!("notification_member saved {:?}", member.chat_id);
                ()
            }
            Err(err) => error!("creation error: {}", err),
        };
    }

    pub fn search(price: u32) -> Vec<NotificationMember> {
        let conn = sqlite_connection_factory::get();

        let mut stmt = conn.prepare("SELECT * FROM notification_members WHERE min_price <= :price and max_price >= :price ORDER BY chat_id DESC;").unwrap();

        let iter = stmt
            .query_map(named_params! {":price": price}, |row| {
                log::info!("{:?}", row);
                let chat_id: String = row.get(1)?;
                let min_price: u32 = row.get(2)?;
                let max_price: u32 = row.get(3)?;
                Ok(NotificationMember {
                    id: row.get(0)?,
                    min_price: min_price,
                    max_price: max_price,
                    chat_id: chat_id.parse::<i64>().unwrap(),
                })
            })
            .unwrap();

        let mut v: Vec<NotificationMember> = Vec::new();

        for res in iter {
            log::info!("{:?}", res);
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

    pub fn delete(chat_id: i64) -> () {
        let conn = sqlite_connection_factory::get();

        let mut stmt = conn
            .prepare("DELETE FROM notification_members WHERE chat_id = :chat_id")
            .unwrap();

        stmt.execute(named_params! {
            ":chat_id": chat_id,
        })
        .unwrap();

        info!("{} notification_member removed", chat_id);

    }
}