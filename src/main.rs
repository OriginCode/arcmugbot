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

/// maimai difficulties
#[derive(Deserialize, Debug)]
enum Difficulty {
    Easy,
    Advanced,
    Expert,
    Master,
    ReMaster,
}

impl fmt::Display for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Difficulty::Easy => write!(f, "Easy"),
            Difficulty::Advanced => write!(f, "Advanced"),
            Difficulty::Expert => write!(f, "Expert"),
            Difficulty::Master => write!(f, "Master"),
            Difficulty::ReMaster => write!(f, "Re:Master"),
        }
    }
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
    life: u32,
    status: Status,
}

type Courses = Vec<Course>;
type Records = HashMap<i64, HashMap<u32, Record>>;

/// Calculate the life remains for a course
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
        Command::Ping => cx.answer("pong!").await?,
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

            records
                .entry(user)
                .and_modify(|r| {
                    // update record
                    r.entry(level)
                        .and_modify(|record| {
                            record.life = remain;
                            record.status = status;
                        })
                        .or_insert(Record {
                            life: remain,
                            status,
                        });
                })
                .or_insert_with(|| {
                    // new record
                    let mut record = HashMap::new();
                    record.insert(
                        level,
                        Record {
                            life: remain,
                            status,
                        },
                    );
                    record
                });
            serde_json::to_writer_pretty(
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
        Command::Score { level } => {
            let records: Records =
                serde_json::from_slice(&fs::read(format!("./records-{}.json", get_date()))?)?;
            let courses: Courses =
                serde_json::from_slice(&fs::read(format!("./courses-{}.json", get_date()))?)?;
            // get user id
            let user = cx
                .update
                .from()
                .ok_or_else(|| ParseError::Custom("invalid user".into()))?
                .id;
            if let Some(user_record) = records.get(&user) {
                if let Some(r) = user_record.get(&level) {
                    cx.answer(format!(
                        "Life: {}/{}\n{}",
                        r.life,
                        courses[level as usize - 1].life,
                        r.status
                    ))
                    .await?;
                    return Ok(());
                }
            }
            cx.answer("Record does not exist!").await?
        }
        Command::Query { level } => {
            // print course information
            // For example:
            // Life: 500
            //
            // Song1 Master 14
            // Song2 Re:Master 14
            // Song3 Re:Master 14+
            // Song4 Re:Master 15
            let courses: Courses =
                serde_json::from_slice(&fs::read(format!("./courses-{}.json", get_date()))?)?;
            let course = &courses[level as usize - 1];
            let mut output = format!("Life: {}\n", course.life);
            for song in course.songs.iter() {
                output = format!(
                    "{}\n{} {} {}",
                    output, song.title, song.difficulty, song.level
                );
            }
            cx.answer(output).await?
        }
    };

    Ok(())
}

async fn run() -> Result<()> {
    teloxide::enable_logging!();
    log::info!("Starting arcmugbot...");

    let bot = Bot::new(TOKEN).auto_send();
    let bot_name = "arcmugbot".to_owned();

    teloxide::commands_repl(bot, bot_name, answer).await;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    run().await?;

    Ok(())
}
