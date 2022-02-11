use anyhow::Result;
use reqwest::{Client, Response};

use crate::ARCANA_TOKEN;

pub async fn build_get_request(url: &str) -> Result<Response> {
    Ok(Client::builder()
        .user_agent("curl/7.81.0")
        .build()?
        .get(url)
        .bearer_auth(ARCANA_TOKEN)
        .send()
        .await?)
}

#[cfg(test)]
mod tests {
    use crate::arcana::build_get_request;

    #[tokio::test]
    async fn test_get() {
        println!(
            "{:?}",
            build_get_request("https://arcana.nu/api/v1/")
                .await
                .unwrap()
                .text()
                .await
                .unwrap()
        )
    }
}

pub mod iidx;
