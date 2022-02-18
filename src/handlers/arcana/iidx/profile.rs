use std::error::Error;
use teloxide::{prelude2::*, types::ParseMode, utils::markdown::*};

use super::get_profiles;

pub async fn profile(
    bot: AutoSend<Bot>,
    message: Message,
    version: u32,
    param: &str,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let profiles = get_profiles(version, param).await?;
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
    bot.send_message(message.chat.id, output)
        .parse_mode(ParseMode::MarkdownV2)
        .reply_to_message_id(message.id)
        .await?;

    Ok(())
}
