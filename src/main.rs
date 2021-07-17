use anyhow::Result;
use chrono::{Datelike, Utc};
use chrono_tz::{Asia::Shanghai, Tz};
use commands::{Command, Results};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, fmt, fs};
use teloxide::{
    prelude::*,
    types::ParseMode,
    utils::{
        command::{BotCommand, ParseError},
        markdown::*,
    },
};

mod commands;

const TOKEN: &str = "";
const TZ: &Tz = &Shanghai;

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
    life: u32,
    songs: Vec<Song>,
}

/// An enum showing if the course is passed
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
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
    level: u32,
    life: u32,
    status: Status,
}

type Courses = Vec<Course>;
type Records = HashMap<i64, Record>;

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

/// Get the date of the current month
fn get_date() -> String {
    let datetime = Utc::today().with_timezone(TZ);
    format!("{}-{}", datetime.year(), datetime.month())
}

/// Parse Telegram commands
async fn answer(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    command: Command,
) -> std::result::Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Help => cx.answer(Command::descriptions()).await?,
        Command::Submit { level, results } => {
            let mut records: Records =
                serde_json::from_slice(&fs::read(format!("./records-{}.json", get_date()))?)?;
            let courses: Courses =
                serde_json::from_slice(&fs::read(format!("./courses-{}.json", get_date()))?)?;
            if level as usize > courses.len() {
                cx.answer("Invalid course level!").await?;
                return Ok(());
            }
            // get user id
            let user = cx
                .update
                .from()
                .ok_or_else(|| ParseError::Custom("invalid user".into()))?
                .id;

            let life = courses[level as usize - 1].life;
            let (remain, status) = parse_score(life, results)?;

            if let Some(r) = records.get_mut(&user) {
                // update record
                r.level = level;
                r.life = remain;
                r.status = status;
            } else {
                // new record
                records.insert(
                    user,
                    Record {
                        level,
                        life: remain,
                        status,
                    },
                );
            }
            serde_json::to_writer(
                fs::File::create(format!("./records-{}.json", get_date()))?,
                &records,
            )?;

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
        Command::Score { level: _ } | Command::Query { level: _ } => cx.answer("WIP").await?,
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
