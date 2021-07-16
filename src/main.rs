use anyhow::Result;
use commands::{Command, Results};
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt};
use teloxide::{
    prelude::*,
    types::ParseMode,
    utils::{command::BotCommand, markdown::*},
};

mod commands;

const TOKEN: &str = "";

// maimai difficulties
#[derive(Deserialize, Debug)]
enum Difficulty {
    Easy,
    Advanced,
    Expert,
    Master,
    ReMaster,
}

#[derive(Deserialize, Debug)]
struct Song {
    title: String,
    difficulty: Difficulty,
    level: String,
}

#[derive(Deserialize, Debug)]
struct Course {
    level: u32,
    life: u32,
    songs: Vec<Song>,
}

/// An enum showing if the course is passed
#[derive(Serialize, Deserialize, Debug)]
enum Status {
    Passed,
    Failed,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Passed => write!(f, "Passed"),
            Status::Failed => write!(f, "Failed"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Record {
    course_level: u32,
    life: u32,
    status: Status,
}

/// Use to calculate the life remains for a course
fn parse_score(life: u32, results: Results) -> Result<(u32, Status)> {
    let mut life = life as i32;
    for result in results {
        life -= result.0 as i32 * 2 + result.1 as i32 * 3 + result.2 as i32 * 5;
    }
    if life < 0 {
        life = 0;
    }

    Ok((
        life as u32,
        if life > 0 {
            Status::Passed
        } else {
            Status::Failed
        },
    ))
}

/// Parse Telegram commands
async fn answer(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    command: Command,
) -> std::result::Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Help => cx.answer(Command::descriptions()).await?,
        Command::Submit { level, results } => {
            let life = 500;
            let (remain, status) = parse_score(life, results)?;
            cx.answer(format!(
                "{}\nCourse Level: {}\nLife: {}/{}\n{}",
                code_inline("[DBG] [WIP]"),
                level,
                remain,
                life,
                status,
            ))
            .parse_mode(ParseMode::MarkdownV2)
            .await?
        }
    };

    Ok(())
}

async fn run() -> Result<()> {
    teloxide::enable_logging!();
    log::info!("Starting arcmugbot...");

    let bot = Bot::new(TOKEN).auto_send();
    let bot_name = "Arcade MUG Bot".to_owned();

    teloxide::commands_repl(bot, bot_name, answer).await;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    run().await?;

    Ok(())
}
