use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DonmaiImage {
    pub id: u32,
    pub file_url: String,
}

pub async fn get_random_picture(client: &Client, search_tags: String) -> Result<DonmaiImage> {
    let image: DonmaiImage = client
        .get(format!(
            "https://donmai.moe/posts/random.json?tags={}",
            search_tags
        ))
        .send()
        .await?
        .json()
        .await
        .context("Sorry, I couldn't find anything")?;

    Ok(image)
}
