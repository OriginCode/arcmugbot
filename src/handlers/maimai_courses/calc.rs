use std::error::Error;
use teloxide::prelude2::*;

use crate::maimai_courses::{Status, Submission};

/// Calculate the life remains
pub async fn calc_life(submission: &Submission) -> (u32, Status) {
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

pub async fn calc(
    bot: AutoSend<Bot>,
    message: Message,
    submission: &Submission,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let (remain, status) = calc_life(submission).await;
    bot.send_message(
        message.chat.id,
        format!("Life: {}/{}\n{}", remain, submission.life, status),
    )
    .reply_to_message_id(message.id)
    .await?;
    Ok(())
}
