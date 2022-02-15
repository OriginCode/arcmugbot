use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fmt;

use super::get_resp;

#[derive(Debug, Deserialize, Serialize)]
pub enum Difficulty {
    #[serde(rename = "BEGINNER")]
    Beginner,
    #[serde(rename = "NORMAL")]
    Normal,
    #[serde(rename = "HYPER")]
    Hyper,
    #[serde(rename = "ANOTHER")]
    Another,
    #[serde(rename = "BLACK")]
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

#[derive(Debug, Deserialize, Serialize)]
pub enum PlayStyle {
    #[serde(rename = "SINGLE")]
    Single,
    #[serde(rename = "DOUBLE")]
    Double,
}

impl fmt::Display for PlayStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlayStyle::Single => write!(f, "SP"),
            PlayStyle::Double => write!(f, "DP"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Chart {
    #[serde(rename = "_id")]
    pub id: String,
    pub bpm_max: f32,
    pub bpm_min: f32,
    pub difficulty: Difficulty,
    pub music_id: String,
    pub notes: u32,
    pub play_style: PlayStyle,
    pub rating: u32,
}

impl Default for Chart {
    fn default() -> Self {
        Self {
            id: String::new(),
            bpm_max: 0.0,
            bpm_min: 0.0,
            difficulty: Difficulty::Beginner,
            music_id: String::new(),
            notes: 0,
            play_style: PlayStyle::Single,
            rating: 0,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct ChartResp {
    #[serde(rename = "_items")]
    items: Vec<Chart>,
}

pub async fn get_charts(version: u32, music_id: &str) -> Result<Vec<Chart>> {
    let request = get_resp(version, "charts/", &[("music_id", music_id)]).await?;
    let chart_resp: ChartResp = request.json().await?;
    Ok(chart_resp.items)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_profile() {
        println!("{:?}", get_charts(28, "G6vGmV2XC2Y").await.unwrap())
    }
}
