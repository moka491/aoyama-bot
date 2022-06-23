use poise::serenity_prelude::{Color, CreateEmbed};

use crate::core::context::ContextData;

pub fn error_embed(e: &mut CreateEmbed, err: anyhow::Error) -> &mut CreateEmbed {
    e.color(Color::DARK_RED)
        .title("Something happened")
        .description(err.to_string())
}

pub async fn on_error(error: poise::FrameworkError<'_, ContextData, anyhow::Error>) {
    match error {
        poise::FrameworkError::Setup { error } => {
            tracing::error!(?error, "failed to setup bot")
        }

        poise::FrameworkError::Command { error, ctx } => {
            tracing::error!(command = ctx.command().name, ?error, "error in command");

            let _ = ctx
                .send(|m| m.embed(|embed| error_embed(embed, error)))
                .await;
        }

        error => {
            tracing::error!(?error, "calling default error handler");

            if let Err(e) = poise::builtins::on_error(error).await {
                tracing::error!(?e, "error in default error handler")
            }
        }
    }
}
