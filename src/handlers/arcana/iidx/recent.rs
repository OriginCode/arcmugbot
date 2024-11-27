use std::error::Error;
use teloxide::{prelude::*, types::ParseMode, types::ReplyParameters, utils::markdown::*};

use super::get_profiles;
use crate::arcana::iidx::{get_charts, get_most_recent, get_music, Chart};

pub async fn recent(
    bot: Bot,
    message: Message,
    version: u32,
    param: &str,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut profiles = get_profiles(version, param).await?;
    let mut output = "Not found".to_owned();
    if let Some(p) = profiles.pop() {
        if let Some(r) = get_most_recent(version, &p.id).await? {
            let music = get_music(28, &r.music_id).await?.unwrap();
            let mut chart = Chart::default();
            for c in get_charts(28, &r.music_id).await? {
                if c.id == r.chart_id {
                    chart = c;
                }
            }
            output = format!(
                "{}\n{} {} {}\n\nLamp: {}\nEX Score: {}\nMiss Count: {}\n\nTimestamp: {}",
                escape(&music.title),
                chart.play_style,
                chart.difficulty,
                chart.rating,
                r.lamp,
                r.ex_score,
                r.miss_count.unwrap_or_default(),
                escape(&r.timestamp),
            )
        }
    }
    bot.send_message(message.chat.id, output)
        .parse_mode(ParseMode::MarkdownV2)
        .reply_parameters(ReplyParameters::new(message.id))
        .await?;

    Ok(())
}
