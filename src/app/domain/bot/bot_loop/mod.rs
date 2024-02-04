use teloxide::{prelude::*, utils::command::BotCommands};

use super::commands::{get_report_avg_price_by_model::get_report_avg_price_by_model, upsert_notification_member::upsert_notification_member, remove_notification_member::remove_notification_member};

pub async fn do_loop() {
    log::info!("Starting command bot...");
    let bot = Bot::from_env();

    Command::repl(bot, answer).await;
}

/// These commands are supported:
#[derive(Clone, BotCommands)]
#[command(rename_rule = "lowercase")]
enum Command {
    #[command(description = "Все команды")]
    Help,
    #[command(description = "Средние цены машины")]
    CarAVG (String),
    #[command(description = "Начать отслеживание по ценам: min max", parse_with = "split")]
    StartMember { min_price: u32, max_price: u32 },
    #[command(description = "Остановить отслеживание по ценам")]
    StopMember,
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::CarAVG (model ) => {
            bot.send_message(msg.chat.id, get_report_avg_price_by_model(&model)).await?
        }
        Command::StartMember { min_price, max_price } => {
            bot.send_message(msg.chat.id, upsert_notification_member(min_price, max_price, msg.chat.id.0))
                .await?
        }
        Command::StopMember => {
            bot.send_message(msg.chat.id, remove_notification_member(msg.chat.id.0))
                .await?
        }
    };

    Ok(())
}