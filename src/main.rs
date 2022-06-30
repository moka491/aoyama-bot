mod api;
mod commands;
mod core;

use crate::core::{context::init_context, error_handler::on_error};

use anyhow::Result;
use dotenv::dotenv;
use poise::serenity_prelude as serenity;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let framework = poise::Framework::build()
        .token(std::env::var("BOT_TOKEN").expect("Missing BOT_TOKEN"))
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::core::help(),
                commands::anilist::anime(),
                commands::anilist::manga(),
                commands::utilities::summon(),
                commands::admin::register_commands(),
            ],
            on_error: |error| Box::pin(on_error(error)),
            ..Default::default()
        })
        .intents(serenity::GatewayIntents::non_privileged())
        .user_data_setup(move |_, _, _| Box::pin(init_context()))
        .build()
        .await?;

    framework.start_autosharded().await?;

    Ok(())
}
