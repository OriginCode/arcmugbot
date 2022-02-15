use anyhow::Result;
use lazy_static::lazy_static;
use reqwest::{Response, Url};
use serde::Serialize;

use super::build_get_request;

lazy_static! {
    static ref IIDX_URL: Url = Url::parse("https://arcana.nu/api/v1/iidx/").unwrap();
}

async fn get_resp<T: Serialize + ?Sized>(
    version: u32,
    category: &str,
    args: &T,
) -> Result<Response> {
    Ok(
        build_get_request(IIDX_URL.join(&format!("{}/", version))?.join(category)?)
            .await?
            .query(args)
            .send()
            .await?,
    )
}

pub mod chart;
pub mod music;
pub mod profile;
pub mod score_history;
