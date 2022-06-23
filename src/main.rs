mod api;
mod commands;
mod core;

use commands::*;

use anyhow::Result;
use dotenv::dotenv;
use poise::serenity_prelude as serenity;
use reqwest::Client;
use tracing::instrument;

#[derive(Debug)]
pub struct ContextData {
    pub client: Client,
}

pub type CommandContext<'a> = poise::Context<'a, ContextData, anyhow::Error>;

async fn init_context() -> Result<ContextData> {
    let client: Client = reqwest::Client::new();

    Ok(ContextData { client })
}

#[tokio::main]
#[instrument]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let framework = poise::Framework::build()
        .options(poise::FrameworkOptions {
            commands: vec![
                anilist::anime(),
                anilist::manga(),
                thread::ping(),
                admin::register(),
            ],
            ..Default::default()
        })
        .token(std::env::var("BOT_TOKEN").expect("Missing BOT_TOKEN"))
        .intents(serenity::GatewayIntents::non_privileged())
        .user_data_setup(move |_, _, _| Box::pin(init_context()));

    framework.run().await?;

    Ok(())
}
