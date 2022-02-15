use anyhow::Result;
use reqwest::{Client, IntoUrl, RequestBuilder};

use crate::ARCANA_TOKEN;

pub async fn build_get_request(url: impl IntoUrl) -> Result<RequestBuilder> {
    Ok(Client::builder()
        .user_agent("curl/7.81.0")
        .build()?
        .get(url)
        .bearer_auth(ARCANA_TOKEN))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get() {
        println!(
            "{:?}",
            build_get_request("https://arcana.nu/api/v1/")
                .await
                .unwrap()
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap()
        )
    }
}

pub mod iidx;
