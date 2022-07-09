use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DonmaiImage {
    pub id: u32,
    pub file_url: String,
    pub large_file_url: String,
}

pub async fn get_random_picture(client: &Client, tags: Option<String>) -> Result<DonmaiImage> {

    let url: String = match tags {
        Some(t) => format!(
            "https://donmai.moe/posts/random.json?tags={}",
            t
        ),
        None => "https://donmai.moe/posts/random.json".into()
    };

    let image: DonmaiImage = client
        .get(url)
        .send()
        .await?
        .json()
        .await
        .context("Sorry, I couldn't find anything")?;

    Ok(image)
}
