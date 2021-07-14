use anyhow::Result;
use commands::Command;
use teloxide::prelude::*;
use std::error::Error;

mod commands;

const TOKEN: &str = "";

async fn answer(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    command: Command,
) -> std::result::Result<(), Box<dyn Error + Send + Sync>> {
    Ok(())
}

async fn run() -> Result<()> {
    teloxide::enable_logging!();
    log::info!("Starting arcmugbot...");

    let bot = Bot::new(TOKEN).auto_send();
    let bot_name = "Arcade MUG Bot".to_owned();

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    run().await?;

    Ok(())
}
