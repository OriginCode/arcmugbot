use std::error::Error;
use teloxide::{prelude::*, types::ParseMode, utils::markdown::*};

const MAX_SCORE: u32 = 1010000;

pub async fn tolerance_calc(
    bot: AutoSend<Bot>,
    message: Message,
    notes: u32,
    target: &str,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let single = (MAX_SCORE / notes) as f32;
    let single_justice = (single as f32) / 1.01;
    let single_attack = (single as f32) / 2.02;

    let (target, target_score) = match target.to_lowercase().as_str() {
        "sss+" => ("SSS+", 1009000),
        "sss" => ("SSS", 1007500),
        "ss+" => ("SS+", 1005000),
        _ => ("SS", 1000000),
    };

    let total_justice = (MAX_SCORE - target_score) as f32 / (single - single_justice);
    let total_attack = (MAX_SCORE - target_score) as f32 / (single - single_attack);

    bot.send_message(
        message.chat.id,
        format!(
            "For target {} we can have {} justice(s) or {} attack(s)\nJustice: -{}, Attack: -{}, Miss: -{}",
            target,
            total_justice,
            total_attack,
            single - single_justice,
            single - single_attack,
            single,
        ),
    )
    .reply_to_message_id(message.id)
    .send()
    .await?;

    Ok(())
}
