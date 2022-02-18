use chrono::{Datelike, Utc};
use chrono_tz::Tz;
use commands::Command;
use lazy_static::lazy_static;
use std::error::Error;
use teloxide::{prelude2::*, utils::command::BotCommand};
use tokio::fs;

mod arcana;
mod commands;
mod handlers;
mod macros;
mod maimai_courses;

use maimai_courses::{Courses, Records};

const ABOUT: &str =
    "Arcade MUG Bot, designed by OriginCode.\nGitHub: https://github.com/OriginCode/arcmugbot";
const TOKEN: &str = "";
const ARCANA_TOKEN: &str = "";
const TZ: Tz = chrono_tz::Asia::Shanghai;

lazy_static! {
    pub static ref DATE: String = format!("{}-{}", Utc::today().with_timezone(&TZ).year(), 1);
}

/// Parse Telegram commands
async fn answer(
    bot: AutoSend<Bot>,
    message: Message,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // load files
    let mut records: Records =
        serde_json::from_slice(&fs::read(format!("./records-{}.json", *DATE)).await?)?;
    let courses: Courses =
        serde_json::from_slice(&fs::read(format!("./courses-{}.json", *DATE)).await?)?;
    match command {
        Command::Ping => {
            bot.send_message(message.chat.id, "pong!").await?;
        }
        Command::Help => {
            bot.send_message(message.chat.id, Command::descriptions())
                .reply_to_message_id(message.id)
                .await?;
        }
        Command::About => {
            bot.send_message(message.chat.id, ABOUT).await?;
        }
        Command::Calc { submission } => {
            handlers::maimai_courses::calc(bot, message, &submission).await?
        }
        Command::CalcCustom { submission } => {
            handlers::maimai_courses::calc(bot, message, &submission).await?
        }
        Command::Submit { level, results } => {
            handlers::maimai_courses::submit(bot, message, level, results, &courses, &mut records)
                .await?
        }
        Command::Score { level } => {
            handlers::maimai_courses::score(bot, message, level, &courses, &mut records).await?
        }
        Command::Query { level } => {
            handlers::maimai_courses::query(bot, message, level, &courses).await?
        }
        Command::Passed => {
            handlers::maimai_courses::passed(bot, message, &courses, &records).await?
        }
        Command::Rank { level } => {
            handlers::maimai_courses::rank(bot, message, level, &courses, &mut records).await?
        }
        Command::IIDXProfile { version, param } => {
            handlers::arcana::iidx::profile(bot, message, version, &param).await?
        }
        Command::IIDXMusic { version, title } => {
            handlers::arcana::iidx::music(bot, message, version, &title).await?
        }
        Command::IIDXRecent { version, param } => {
            handlers::arcana::iidx::recent(bot, message, version, &param).await?
        }
    };

    Ok(())
}

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("Starting arcmugbot...");

    let bot = Bot::new(TOKEN).auto_send();

    teloxide::repls2::commands_repl(bot, answer, Command::ty()).await;
}
