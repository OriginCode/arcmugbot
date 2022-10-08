use std::error::Error;
use teloxide::{
    prelude::*,
    types::ParseMode,
    utils::{command::ParseError, markdown::*},
};

use crate::maimai_courses::{course::Courses, record::Records};

pub async fn score(
    bot: Bot,
    message: Message,
    level: u32,
    courses: &Courses,
    records: &mut Records,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // print user record
    // For example:
    //
    // Life: 245/900
    // Passed
    if level as usize > courses.len() || level == 0 {
        bot.send_message(message.chat.id, "Invalid course level!")
            .reply_to_message_id(message.id)
            .await?;
        return Ok(());
    }
    // get user id
    let user = message
        .from()
        .ok_or_else(|| ParseError::Custom("invalid user".into()))?
        .id
        .0;
    if let Some(user_record) = records.get(&user) {
        if let Some(r) = user_record.records.get(&level) {
            let course = &courses[level as usize - 1];
            bot.send_message(
                message.chat.id,
                format!(
                    "{} Life: {}/{} {}",
                    bold(&course.name),
                    r.life,
                    course.life,
                    r.status
                ),
            )
            .parse_mode(ParseMode::MarkdownV2)
            .reply_to_message_id(message.id)
            .await?;
            return Ok(());
        }
    }
    bot.send_message(message.chat.id, "Record does not exist!")
        .reply_to_message_id(message.id)
        .await?;

    Ok(())
}
