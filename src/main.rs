use anyhow::Result;
use chrono::{Datelike, Utc};
use chrono_tz::Tz;
use commands::{Command, Results};
use indexmap::IndexMap;
use std::{
    collections::{HashMap, VecDeque},
    error::Error,
};
use teloxide::{
    prelude::*,
    types::ParseMode,
    utils::{
        command::{BotCommand, ParseError},
        markdown::*,
    },
    RequestError,
};
use tokio::fs;

mod arcana;
mod commands;
mod course;
mod macros;
mod record;

use arcana::*;
use course::Courses;
use record::{Record, Records, Status, UserRecords};

const ABOUT: &str =
    "Arcade MUG Bot, designed by OriginCode.\nGitHub: https://github.com/OriginCode/arcmugbot";
const TOKEN: &str = "";
const ARCANA_TOKEN: &str = "";
const TZ: Tz = chrono_tz::Asia::Shanghai;
const RULE: [u32; 3] = [2, 3, 5];

pub struct Submission {
    life: u32,
    heal: u32,
    rule: [u32; 3],
    results: Results,
}

impl Default for Submission {
    fn default() -> Self {
        Self {
            life: 900,
            heal: 20,
            rule: RULE,
            results: VecDeque::new(),
        }
    }
}

/// Calculate the life remains
async fn calc_life(submission: &Submission) -> (u32, Status) {
    let mut remains = submission.life;
    for result in submission.results.iter() {
        if let Some(val) = remains.checked_sub(
            result
                .iter()
                .zip(submission.rule.iter())
                .map(|x| x.0 * x.1)
                .sum(),
        ) {
            remains = val;
            remains += submission.heal;
        } else {
            return (0, Status::Failed);
        }
    }
    remains -= submission.heal;
    if remains > submission.life {
        remains = submission.life;
    }

    (remains, Status::Passed)
}

#[inline]
async fn calc_cmd(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    submission: &Submission,
) -> Result<Message, RequestError> {
    let (remain, status) = calc_life(submission).await;
    cx.reply_to(format!("Life: {}/{}\n{}", remain, submission.life, status))
        .await
}

/// Get the date of the current month
async fn get_date() -> String {
    let datetime = Utc::today().with_timezone(&TZ);
    format!("{}-{}", datetime.year(), 1)
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
        Command::Calc { submission } => calc_cmd(cx, &submission).await?,
        Command::CalcCustom { submission } => calc_cmd(cx, &submission).await?,
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
            let (remain, status) = calc_life(&Submission {
                life,
                heal: course.heal,
                rule: RULE,
                results,
            })
            .await;

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
            let mut output = format!(
                "{} Life: {} Heal: {}\n",
                bold(&course.name),
                course.life,
                course.heal
            );
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
        Command::IIDXProfile { version, param } => {
            let dj_name_profiles = iidx::profile::get_profile(version, &param).await?;
            let profiles = if !dj_name_profiles.is_empty() {
                dj_name_profiles
            } else {
                iidx::profile::get_profile_id(version, &param).await?
            };
            let output = profiles
                .iter()
                .map(|profile| {
                    let sp = &profile.sp;
                    let dp = &profile.dp;
                    format!(
                        "DJ NAME: {}\nIIDX ID: {}\n\n{}\nDJ POINTS: {}\nPLAYS: {}\n\
                RANKS: {}\n\n{}\nDJ POINTS: {}\nPLAYS: {}\n\
                RANKS: {}",
                        escape(&profile.dj_name),
                        escape(&profile.iidx_id),
                        bold("SP"),
                        sp.dj_points,
                        sp.plays,
                        if let Some(ranks) = &sp.rank {
                            ranks
                        } else {
                            "NULL"
                        },
                        bold("DP"),
                        dp.dj_points,
                        dp.plays,
                        if let Some(ranks) = &dp.rank {
                            ranks
                        } else {
                            "NULL"
                        }
                    )
                })
                .collect::<Vec<String>>()
                .join("\n------\n");
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
