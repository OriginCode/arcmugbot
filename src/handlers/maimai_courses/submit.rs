use std::{collections::HashMap, error::Error};
use teloxide::{
    prelude::*,
    types::ParseMode,
    utils::{command::ParseError, markdown::*},
};

use super::calc::calc_life;
use crate::{
    commands::Results,
    maimai_courses::{
        course::Courses,
        record::{Record, Records},
        submission::Submission,
        UserRecords, RULE,
    },
    DATE,
};

pub async fn submit(
    bot: Bot,
    message: Message,
    level: u32,
    results: Results,
    courses: &Courses,
    records: &mut Records,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if level as usize > courses.len() || level == 0 {
        bot.send_message(message.chat.id, "Invalid course level!")
            .reply_to_message_id(message.id)
            .await?;
        return Ok(());
    }
    // get user id
    let user = message
        .from()
        .ok_or_else(|| ParseError::Custom("invalid user".into()))?;
    let course = &courses.get(level as usize - 1).unwrap();
    let life = course.life;
    let (remain, status) = calc_life(&Submission {
        life,
        heal: course.heal,
        rule: RULE,
        results,
    })
    .await;

    records
        .entry(user.id.0)
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
        std::fs::File::create(format!("./records-{}.json", *DATE))?,
        &records,
    )?;

    bot.send_message(
        message.chat.id,
        format!(
            "{}\n{} Life: {}/{} {}",
            escape("Submitted!"),
            bold(&course.name),
            remain,
            life,
            status,
        ),
    )
    .parse_mode(ParseMode::MarkdownV2)
    .reply_to_message_id(message.id)
    .await?;

    Ok(())
}
