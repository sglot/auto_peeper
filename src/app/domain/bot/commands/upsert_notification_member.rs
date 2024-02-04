use crate::app::domain::models::{
    notification_member::NotificationMember,
    notification_member_repository::NotificationMemberRepository,
};

pub fn upsert_notification_member(min_price: u32, max_price: u32, chat_id: i64) -> String {
    let mut member = NotificationMember::new();
    member.min_price = min_price;
    member.max_price = max_price;
    member.chat_id = chat_id;

    NotificationMemberRepository::upsert(&mut member);

    "Настройки применены".to_string()
}
