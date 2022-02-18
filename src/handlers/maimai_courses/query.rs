use std::error::Error;
use teloxide::{prelude2::*, types::ParseMode, utils::markdown::*};

use crate::maimai_courses::course::Courses;

pub async fn query(
    bot: AutoSend<Bot>,
    message: Message,
    level: u32,
    courses: &Courses,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // print course information
    // For example:
    // Life: 500
    //
    // Song1 Master 14
    // Song2 Re:Master 14
    // Song3 Re:Master 14+
    // Song4 Re:Master 15
    if level as usize > courses.len() || level == 0 {
        bot.send_message(message.chat.id, "Invalid course level!")
            .reply_to_message_id(message.id)
            .await?;
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
    bot.send_message(message.chat.id, output)
        .parse_mode(ParseMode::MarkdownV2)
        .reply_to_message_id(message.id)
        .await?;

    Ok(())
}
