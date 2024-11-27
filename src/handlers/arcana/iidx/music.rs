use std::error::Error;
use teloxide::{prelude::*, types::ParseMode, types::ReplyParameters, utils::markdown::*};

use crate::arcana::iidx::{get_charts, get_music_folder};

pub async fn music(
    bot: Bot,
    message: Message,
    version: u32,
    title: &str,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let music = get_music_folder(version, version).await?;
    let mut output = "Not found".to_owned();
    for m in music {
        if m.title == title {
            let charts = get_charts(version, &m.id).await?;
            output = format!(
                "{}\n{}\n{}\n\nDifficulties:\n",
                escape(&m.genre),
                escape(&m.title),
                escape(&m.artist)
            );
            for c in charts {
                output.push_str(&format!("{} {} {}\n", c.play_style, c.difficulty, c.rating));
            }
        }
    }
    bot.send_message(message.chat.id, output)
        .parse_mode(ParseMode::MarkdownV2)
        .reply_parameters(ReplyParameters::new(message.id))
        .await?;

    Ok(())
}
