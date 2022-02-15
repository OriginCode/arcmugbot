use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fmt;

use super::get_resp;

#[derive(Debug, Deserialize, Serialize)]
pub enum Lamp {
    #[serde(rename = "NO_PLAY")]
    NoPlay,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "ASSIST_CLEAR")]
    AssistClear,
    #[serde(rename = "EASY_CLEAR")]
    EasyClear,
    #[serde(rename = "CLEAR")]
    Clear,
    #[serde(rename = "HARD_CLEAR")]
    HardClear,
    #[serde(rename = "EX_HARD_CLEAR")]
    ExHardClear,
    #[serde(rename = "FULL_COMBO")]
    FullCombo,
}

impl fmt::Display for Lamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Lamp::NoPlay => write!(f, "NO PLAY"),
            Lamp::Failed => write!(f, "FAILED"),
            Lamp::AssistClear => write!(f, "ASSIST CLEAR"),
            Lamp::EasyClear => write!(f, "EASY CLEAR"),
            Lamp::Clear => write!(f, "CLEAR"),
            Lamp::HardClear => write!(f, "HARD CLEAR"),
            Lamp::ExHardClear => write!(f, "EX HARD CLEAR"),
            Lamp::FullCombo => write!(f, "FULL COMBO"),
        }
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ScoreHistory {
    #[serde(rename = "_id")]
    pub id: String,
    pub chart_id: String,
    pub ex_score: u32,
    pub lamp: Lamp,
    pub miss_count: Option<u32>,
    pub music_id: String,
    pub profile_id: String,
    pub raised: bool,
    pub status: Lamp,
    pub timestamp: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct ScoreHistoryResp {
    #[serde(rename = "_items")]
    items: Vec<ScoreHistory>,
}

pub async fn get_score_history(version: u32, profile_id: &str) -> Result<Vec<ScoreHistory>> {
    let request = get_resp(version, "score_history/", &[("profile_id", profile_id)]).await?;
    let profile_resp: ScoreHistoryResp = request.json().await?;
    Ok(profile_resp.items)
}

pub async fn get_most_recent(version: u32, profile_id: &str) -> Result<Option<ScoreHistory>> {
    Ok(get_score_history(version, profile_id).await?.pop())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_most_recent() {
        println!("{:?}", get_most_recent(28, "C3PttzgAx6F").await.unwrap())
    }
}
