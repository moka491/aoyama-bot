mod api;
mod commands;
mod core;

use commands::*;

use anyhow::Result;
use dotenv::dotenv;
use poise::serenity_prelude as serenity;
use reqwest::Client;

pub struct ContextData {
    pub client: Client,
}

pub type Context<'a> = poise::Context<'a, ContextData, anyhow::Error>;

async fn init_context() -> Result<ContextData> {
    let client: Client = reqwest::Client::new();

    Ok(ContextData { client })
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let framework = poise::Framework::build()
        .options(poise::FrameworkOptions {
            commands: vec![anilist::anime(), admin::commands::register()],
            ..Default::default()
        })
        .token(std::env::var("BOT_TOKEN").expect("Missing BOT_TOKEN"))
        .intents(serenity::GatewayIntents::non_privileged())
        .user_data_setup(move |_, _, _| Box::pin(init_context()));

    framework.run().await?;

    Ok(())
}
