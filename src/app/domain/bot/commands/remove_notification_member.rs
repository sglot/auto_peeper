use crate::app::domain::models::notification_member_repository::NotificationMemberRepository;

pub fn remove_notification_member(chat_id: i64) -> String {
    NotificationMemberRepository::delete(chat_id);

    "Остановлено".to_string()
}
