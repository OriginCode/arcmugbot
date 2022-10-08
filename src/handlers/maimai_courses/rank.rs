use indexmap::IndexMap;
use std::error::Error;
use teloxide::{prelude::*, types::ParseMode, utils::markdown::*};

use crate::maimai_courses::{Courses, Records, Status};

pub async fn rank(
    bot: AutoSend<Bot>,
    message: Message,
    level: u32,
    courses: &Courses,
    records: &mut Records,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if level as usize > courses.len() || level == 0 {
        bot.send_message(message.chat.id, "Invalid course level!")
            .reply_to_message_id(message.id)
            .await?;
        return Ok(());
    }
    let mut output = bold(&courses[level as usize - 1].name);
    let mut player_records = IndexMap::new();
    for r in records.iter() {
        if let Some(c) = r.1.records.get(&level) {
            if c.status == Status::Passed {
                player_records.insert(&r.1.fullname, c.life);
            }
        }
    }
    if player_records.is_empty() {
        output = format!("{}\n{}", output, escape("No record yet."));
        bot.send_message(message.chat.id, output)
            .parse_mode(ParseMode::MarkdownV2)
            .reply_to_message_id(message.id)
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
    bot.send_message(message.chat.id, output)
        .parse_mode(ParseMode::MarkdownV2)
        .reply_to_message_id(message.id)
        .await?;

    Ok(())
}
