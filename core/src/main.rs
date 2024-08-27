use teloxide::prelude::*;
use teloxide::types::ParseMode;
use teloxide::adaptors::DefaultParseMode;
use dotenvy::dotenv;
use std::env;
use log::info;
use pretty_env_logger::env_logger;

#[tokio::main]
async fn main() {
    // basic ahh vanilla bot
    dotenv().ok();
    env_logger::init();

    info!("Iniciando el bot...");

    let bot_token = env::var("BOT_TOKEN").expect("BOT_TOKEN is not set");

    let bot = Bot::new(bot_token).parse_mode(ParseMode::MarkdownV2);

    teloxide::repl(bot, |message: Message, bot: DefaultParseMode<Bot>| async move {
        bot.send_message(message.chat.id, "Hola, Soy un bot de Teloxide")
            .await?;
        Ok(())
    })
    .await;
}
