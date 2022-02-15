use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::get_resp;

#[derive(Debug, Deserialize, Serialize)]
pub struct Music {
    #[serde(rename = "_id")]
    pub id: String,
    pub artist: String,
    pub folder: u32,
    pub genre: String,
    pub title: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct MusicResp {
    #[serde(rename = "_items")]
    items: Vec<Music>,
}

pub async fn get_music(version: u32, id: &str) -> Result<Option<Music>> {
    let request = get_resp(version, "music/", &[("_id", id)]).await?;
    let mut music_resp: MusicResp = request.json().await?;
    Ok(music_resp.items.pop())
}

pub async fn get_music_folder(version: u32, folder: u32) -> Result<Vec<Music>> {
    let request = get_resp(version, "music/", &[("folder", folder)]).await?;
    let music_resp: MusicResp = request.json().await?;
    Ok(music_resp.items)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_music() {
        println!("{:?}", get_music(28, "G6vGmV2XC2Y").await.unwrap())
    }

    #[tokio::test]
    async fn test_get_music_folder() {
        println!("{:?}", get_music_folder(28, 1).await.unwrap())
    }
}
