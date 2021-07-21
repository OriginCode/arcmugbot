use anyhow::Result;
use chrono::{Datelike, Utc};
use chrono_tz::{Asia::Shanghai, Tz};
use commands::{Command, Results};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, fmt, fs};
use teloxide::{
    prelude::*,
    types::ParseMode,
    utils::command::{BotCommand, ParseError},
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
    name: String,
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
async fn parse_score(life: u32, results: Results) -> Result<(u32, Status)> {
    let mut life = life as i32;
    for result in results {
        life -= result.0 as i32 * 2 + result.1 as i32 * 3 + result.2 as i32 * 5;
    }
    let status = if life < 0 {
        life = 0;
        Status::Failed
    } else {
        Status::Passed
    };

    Ok((life as u32, status))
}

/// Get the date of the current month
async fn get_date() -> String {
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
        Command::Help => cx.reply_to(Command::descriptions()).await?,
        Command::Calc { life, results } => {
            let (remain, status) = parse_score(life, results).await?;
            cx.reply_to(format!("Life: {}/{}\n{}", remain, life, status))
                .await?
        }
        Command::Submit { level, results } => {
            let date = get_date().await;
            let mut records: Records =
                serde_json::from_slice(&fs::read(format!("./records-{}.json", date))?)?;
            let courses: Courses =
                serde_json::from_slice(&fs::read(format!("./courses-{}.json", date))?)?;
            if level as usize > courses.len() || level <= 0 {
                cx.reply_to("Invalid course level!").await?;
                return Ok(());
            }
            // get user id
            let user = cx
                .update
                .from()
                .ok_or_else(|| ParseError::Custom("invalid user".into()))?
                .id;
            let course = &courses[level as usize - 1];
            let life = course.life;
            let (remain, status) = parse_score(life, results).await?;

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
                fs::File::create(format!("./records-{}.json", date))?,
                &records,
            )?;

            cx.reply_to(format!(
                "Submitted!\n{}\nLife: {}/{}\n{}\n",
                course.name, remain, life, status,
            ))
            .parse_mode(ParseMode::MarkdownV2)
            .await?
        }
        Command::Score { level } => {
            // print user record
            // For example:
            //
            // Life: 245/900
            // Passed
            let date = get_date().await;
            let records: Records =
                serde_json::from_slice(&fs::read(format!("./records-{}.json", date))?)?;
            let courses: Courses =
                serde_json::from_slice(&fs::read(format!("./courses-{}.json", date))?)?;
            if level as usize > courses.len() || level <= 0 {
                cx.reply_to("Invalid course level!").await?;
                return Ok(());
            }
            // get user id
            let user = cx
                .update
                .from()
                .ok_or_else(|| ParseError::Custom("invalid user".into()))?
                .id;
            if let Some(user_record) = records.get(&user) {
                if let Some(r) = user_record.get(&level) {
                    let course = &courses[level as usize - 1];
                    cx.reply_to(format!(
                        "{}\nLife: {}/{}\n{}",
                        course.name, r.life, course.life, r.status
                    ))
                    .await?;
                    return Ok(());
                }
            }
            cx.reply_to("Record does not exist!").await?
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
                serde_json::from_slice(&fs::read(format!("./courses-{}.json", get_date().await))?)?;
            if level as usize > courses.len() || level <= 0 {
                cx.reply_to("Invalid course level!").await?;
                return Ok(());
            }
            let course = &courses[level as usize - 1];
            let mut output = format!("{}\nLife: {}\n", course.name, course.life);
            for song in course.songs.iter() {
                output = format!(
                    "{}\n{} {} {}",
                    output, song.title, song.difficulty, song.level
                );
            }
            cx.reply_to(output).await?
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
