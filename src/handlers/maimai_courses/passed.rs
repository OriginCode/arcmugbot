use std::error::Error;
use teloxide::{prelude::*, types::ParseMode, utils::markdown::*};

use crate::maimai_courses::{Courses, Records, Status};

pub async fn passed(
    bot: AutoSend<Bot>,
    message: Message,
    courses: &Courses,
    records: &Records,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut output = String::new();
    for r in records {
        // show all the passed records of players
        let mut passed_courses = String::new();
        for c in r.1.records.iter() {
            if c.1.status == Status::Passed {
                passed_courses = format!(
                    "{} {}",
                    passed_courses,
                    bold(&courses[*c.0 as usize - 1].name)
                );
            }
        }
        output = format!("{}\n{}: {}", output, escape(&r.1.fullname), passed_courses);
    }
    bot.send_message(message.chat.id, output)
        .parse_mode(ParseMode::MarkdownV2)
        .reply_to_message_id(message.id)
        .await?;

    Ok(())
}
