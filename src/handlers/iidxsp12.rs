use fuzzy_matcher::{clangd::ClangdMatcher, FuzzyMatcher};
use priority_queue::PriorityQueue;
use reqwest::get;
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt, hash::Hash};
use teloxide::prelude::*;

#[derive(PartialEq, Eq, Hash, Debug, Deserialize, Serialize)]
pub enum Difficulty {
    #[serde(rename = "B")]
    Beginner,
    #[serde(rename = "N")]
    Normal,
    #[serde(rename = "H")]
    Hyper,
    #[serde(rename = "A")]
    Another,
    #[serde(rename = "L")]
    Leggendaria,
}

impl fmt::Display for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Difficulty::Beginner => write!(f, "BEGINNER"),
            Difficulty::Normal => write!(f, "NORMAL"),
            Difficulty::Hyper => write!(f, "HYPER"),
            Difficulty::Another => write!(f, "ANOTHER"),
            Difficulty::Leggendaria => write!(f, "LEGGENDARIA"),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Deserialize, Serialize)]
struct Song {
    name: String,
    difficulty: Option<Difficulty>,
    normal: String,
    hard: String,
    version: u8,
}

fn pop_same_priority<T: PartialEq + Eq + Hash>(pq: &mut PriorityQueue<T, i64>) -> Vec<T> {
    let mut result = Vec::new();
    let priority = pq.peek().map(|(_, p)| *p);
    if let Some(priority) = priority {
        while let Some((entry, p)) = pq.pop() {
            if p == priority {
                result.push(entry);
            } else {
                break;
            }
        }
    }
    result
}

pub async fn sp12(
    bot: Bot,
    message: Message,
    title: &str,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let table = get("https://iidx-sp12.github.io/songs.json")
        .await?
        .json::<Vec<Song>>()
        .await?;

    let matcher = ClangdMatcher::default();
    let mut pq: PriorityQueue<&Song, i64> = PriorityQueue::new();
    for entry in table.iter() {
        if let Some(score) = matcher.fuzzy_match(&entry.name, title) {
            pq.push(entry, score);
        }
    }

    if !pq.is_empty() {
        let entries = pop_same_priority(&mut pq);
        for entry in entries {
            bot.send_message(
                message.chat.id,
                format!(
                    "Title: {}\nDifficulty: {}\nVersion: {}\n\nNormal: {}\nHard: {}",
                    entry.name,
                    entry
                        .difficulty
                        .as_ref()
                        .map(|d| d.to_string())
                        .unwrap_or("Unknown".to_owned()),
                    entry.version,
                    if entry.normal.is_empty() {
                        "未定"
                    } else {
                        &entry.normal
                    },
                    if entry.hard.is_empty() {
                        "未定"
                    } else {
                        &entry.hard
                    }
                ),
            )
            .reply_to_message_id(message.id)
            .await?;
        }
    } else {
        bot.send_message(message.chat.id, "Not found".to_owned())
            .reply_to_message_id(message.id)
            .await?;
    }

    Ok(())
}
