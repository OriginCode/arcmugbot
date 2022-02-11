use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::arcana::build_get_request;

const PROFILES_URL: &str = "https://arcana.nu/api/v1/iidx/";

#[derive(Debug, Deserialize, Serialize)]
pub struct Score {
    pub dj_points: u32,
    pub plays: u32,
    pub rank: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Profile {
    #[serde(rename = "_id")]
    id: String,
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

pub async fn get_profile(version: u32, dj_name: &str) -> Result<Option<Profile>> {
    let request = build_get_request(
        format!("{}{}/profiles/?dj_name={}", PROFILES_URL, version, dj_name).as_str(),
    )
    .await?;
    let mut profile_resp: ProfileResp = request.json().await?;
    Ok(profile_resp.items.pop())
}

#[cfg(test)]
mod tests {
    use crate::arcana::build_get_request;
    use crate::arcana::iidx::profile::{get_profile, PROFILES_URL};

    #[tokio::test]
    async fn test_get_profile() {
        println!("{:?}", get_profile(28, "ORIGIN").await.unwrap())
    }
}
