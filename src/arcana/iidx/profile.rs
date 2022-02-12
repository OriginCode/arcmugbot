use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::IIDX_URL;
use crate::arcana::build_get_request;

#[derive(Debug, Deserialize, Serialize)]
pub struct Score {
    pub dj_points: u32,
    pub plays: u32,
    pub rank: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Profile {
    #[serde(rename = "_id")]
    pub id: String,
    pub dj_name: String,
    pub iidx_id: String,
    pub sp: Score,
    pub dp: Score,
}

#[derive(Debug, Deserialize, Serialize)]
struct ProfileResp {
    #[serde(rename = "_items")]
    items: Vec<Profile>,
}

pub async fn get_profile(version: u32, dj_name: &str) -> Result<Vec<Profile>> {
    let request = build_get_request(IIDX_URL.join(&format!("{}/", version))?.join("profiles/")?)
        .await?
        .query(&[("dj_name", dj_name)])
        .send()
        .await?;
    let profile_resp: ProfileResp = request.json().await?;
    Ok(profile_resp.items)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_profile() {
        println!("{:?}", get_profile(28, "A").await.unwrap())
    }
}
