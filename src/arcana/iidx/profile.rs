use anyhow::Result;
use reqwest::Response;
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

async fn get_resp<T: Serialize + ?Sized >(version:u32, args: &T) -> Result<Response> {
    Ok(build_get_request(IIDX_URL.join(&format!("{}/", version))?.join("profiles/")?)
        .await?
        .query(args)
        .send()
        .await?)
}

pub async fn get_profile(version: u32, dj_name: &str) -> Result<Vec<Profile>> {
    let request = get_resp(version, &[("dj_name", dj_name)]).await?;
    let profile_resp: ProfileResp = request.json().await?;
    Ok(profile_resp.items)
}

pub async fn get_profile_id(version: u32, iidx_id: &str) -> Result<Vec<Profile>> {
    let request = get_resp(version, &[("iidx_id", iidx_id)]).await?;
    let profile_resp: ProfileResp = request.json().await?;
    Ok(profile_resp.items)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_profile() {
        println!("{:?}", get_profile(28, "1015-0869").await.unwrap())
    }
}
