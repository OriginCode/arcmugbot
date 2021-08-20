use anyhow::Result;
use chrono::{Datelike, Utc};
use chrono_tz::{Asia::Shanghai, Tz};
use commands::{Command, Results};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, fmt};
use teloxide::{
    prelude::*,
    types::ParseMode,
    utils::{
        command::{BotCommand, ParseError},
        markdown::*,
    },
};
use tokio::fs;

mod commands;

const ABOUT: &str =
    "Arcade MUG Bot, designed by OriginCode.\nGitHub: https://github.com/OriginCode/arcmugbot";
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
    heal: u32,
    songs: Vec<Song>,
}

/// An enum showing if the course is passed
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug)]
struct UserRecords {
    fullname: String,
    records: HashMap<u32, Record>,
}

type Courses = Vec<Course>;
type Records = HashMap<i64, UserRecords>;

/// Calculate the life remains for a course
#[inline]
async fn parse_score(life: u32, heal: u32, results: Results) -> Result<(u32, Status)> {
    parse_score_custom(life, heal, &(2, 3, 5), results).await
}

/// Calculate the life remains for a custom rule
async fn parse_score_custom(
    life: u32,
    heal: u32,
    rule: &(u32, u32, u32),
    results: Results,
) -> Result<(u32, Status)> {
    let mut life = life as i32;
    let mut status = Status::Passed;
    for result in results {
        life -= result.0 as i32 * rule.0 as i32
            + result.1 as i32 * rule.1 as i32
            + result.2 as i32 * rule.2 as i32;
        if life < 0 {
            life = 0;
            status = Status::Failed;
            break;
        }
        life += heal as i32;
    }

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
    // load files
    let date = get_date().await;
    let mut records: Records =
        serde_json::from_slice(&fs::read(format!("./records-{}.json", date)).await?)?;
    let courses: Courses =
        serde_json::from_slice(&fs::read(format!("./courses-{}.json", date)).await?)?;
    match command {
        Command::Ping => cx.answer("pong!").await?,
        Command::Help => cx.reply_to(Command::descriptions()).await?,
        Command::About => cx.reply_to(ABOUT).await?,
        Command::Calc {
            life,
            heal,
            results,
        } => {
            let (remain, status) = parse_score(life, heal, results).await?;
            cx.reply_to(format!("Life: {}/{}\n{}", remain, life, status))
                .await?
        }
        Command::CalcCustom {
            life,
            heal,
            results,
        } => {
            let rule = results.get(0).unwrap_or_else(|| &(2, 3, 5));
            let (remain, status) = parse_score_custom(
                life,
                heal,
                rule,
                results[1..].iter().map(|x| *x).collect::<Results>(),
            )
            .await?;
            cx.reply_to(format!("Life: {}/{}\n{}", remain, life, status))
                .await?
        }
        Command::Submit { level, results } => {
            if level as usize > courses.len() || level == 0 {
                cx.reply_to("Invalid course level!").await?;
                return Ok(());
            }
            // get user id
            let user = cx
                .update
                .from()
                .ok_or_else(|| ParseError::Custom("invalid user".into()))?;
            let course = &courses[level as usize - 1];
            let life = course.life;
            let (remain, status) = parse_score(life, course.heal, results).await?;

            records
                .entry(user.id)
                .and_modify(|r| {
                    // update record
                    r.fullname = user.full_name();
                    r.records
                        .entry(level)
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
                    let mut records = HashMap::new();
                    records.insert(
                        level,
                        Record {
                            life: remain,
                            status,
                        },
                    );
                    UserRecords {
                        fullname: user.full_name(),
                        records,
                    }
                });
            serde_json::to_writer_pretty(
                std::fs::File::create(format!("./records-{}.json", date))?,
                &records,
            )?;

            cx.reply_to(format!(
                "{}\n{} Life: {}/{} {}",
                escape("Submitted!"),
                bold(&course.name),
                remain,
                life,
                status,
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
            if level as usize > courses.len() || level == 0 {
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
                if let Some(r) = user_record.records.get(&level) {
                    let course = &courses[level as usize - 1];
                    cx.reply_to(format!(
                        "{} Life: {}/{} {}",
                        bold(&course.name),
                        r.life,
                        course.life,
                        r.status
                    ))
                    .parse_mode(ParseMode::MarkdownV2)
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
            if level as usize > courses.len() || level == 0 {
                cx.reply_to("Invalid course level!").await?;
                return Ok(());
            }
            let course = &courses[level as usize - 1];
            let mut output = format!("{} Life: {}\n", bold(&course.name), course.life);
            for song in course.songs.iter() {
                output = format!(
                    "{}\n{} {} {}",
                    output,
                    escape(&song.title),
                    song.difficulty,
                    escape(&song.level)
                );
            }
            cx.reply_to(output)
                .parse_mode(ParseMode::MarkdownV2)
                .await?
        }
        Command::Passed => {
            let mut output = String::new();
            for r in records {
                // show all the passed records of players
                let mut passed_courses = String::new();
                for c in r.1.records {
                    if c.1.status == Status::Passed {
                        passed_courses = format!(
                            "{} {}",
                            passed_courses,
                            bold(&courses[c.0 as usize - 1].name)
                        );
                    }
                }
                output = format!("{}\n{}: {}", output, escape(&r.1.fullname), passed_courses);
            }
            cx.reply_to(output)
                .parse_mode(ParseMode::MarkdownV2)
                .await?
        }
        Command::Rank { level } => {
            if level as usize > courses.len() || level == 0 {
                cx.reply_to("Invalid course level!").await?;
                return Ok(());
            }
            let mut output = bold(&courses[level as usize - 1].name);
            let mut player_records = IndexMap::new();
            for r in records {
                if let Some(c) = r.1.records.get(&level) {
                    if c.status == Status::Passed {
                        player_records.insert(r.1.fullname, c.life);
                    }
                }
            }
            if player_records.is_empty() {
                output = format!("{}\n{}", output, escape("No record yet."));
                cx.reply_to(output)
                    .parse_mode(ParseMode::MarkdownV2)
                    .await?;
                return Ok(());
            }
            player_records.sort_by(|_, a, _, b| Ord::cmp(b, a)); // top rank
            for r in player_records.iter().enumerate() {
                output = format!(
                    "{}\n{}{} {}: {}",
                    output,
                    r.0 + 1,
                    escape("."),
                    escape(r.1 .0), // player fullname
                    r.1 .1          // life remains
                )
            }
            cx.reply_to(output)
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
    let bot_name = "arcmugbot".to_owned();

    teloxide::commands_repl(bot, bot_name, answer).await;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    run().await?;

    Ok(())
}
