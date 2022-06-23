use anyhow::Result;
use reqwest::Client;

#[derive(Debug)]
pub struct ContextData {
    pub client: Client,
}

pub type CommandContext<'a> = poise::Context<'a, ContextData, anyhow::Error>;

pub async fn init_context() -> Result<ContextData> {
    let client: Client = reqwest::Client::new();

    Ok(ContextData { client })
}
