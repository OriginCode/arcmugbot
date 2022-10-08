use chrono::{Datelike, Utc};
use chrono_tz::Tz;
use commands::Command;
use lazy_static::lazy_static;
use std::error::Error;
use teloxide::{filter_command, prelude::*, types::Update, utils::command::BotCommands};
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
    bot: Bot,
    message: Message,
    command: Command,
    mut records: Records,
    courses: Courses,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Ping => {
            bot.send_message(message.chat.id, "pong!").await?;
        }
        Command::Help => {
            bot.send_message(message.chat.id, Command::descriptions().to_string())
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
        Command::ChuniTolerance { notes, target } => {
            handlers::chuni_tolerance_calc::tolerance_calc(bot, message, notes, &target).await?
        }
    };

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    pretty_env_logger::init();
    log::info!("Starting arcmugbot...");

    let bot = Bot::new(TOKEN);

    // load files
    let records: Records =
        serde_json::from_slice(&fs::read(format!("./records-{}.json", *DATE)).await?)?;
    let courses: Courses =
        serde_json::from_slice(&fs::read(format!("./courses-{}.json", *DATE)).await?)?;

    Dispatcher::builder(
        bot,
        Update::filter_message().branch(filter_command::<Command, _>().endpoint(answer)),
    )
    .dependencies(dptree::deps![records, courses])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;

    Ok(())
}
