use poise::serenity_prelude::{Color, CreateEmbed};

use crate::core::context::ContextData;

pub fn command_error_embed(e: &mut CreateEmbed, err: anyhow::Error) -> &mut CreateEmbed {
    e.color(Color::DARK_RED)
        .title("Ara?")
        .description(err.to_string())
}

pub async fn on_error(error: poise::FrameworkError<'_, ContextData, anyhow::Error>) {
    match error {
        poise::FrameworkError::Command { error, ctx } => {
            tracing::error!(%error, "command error");

            let _ = ctx
                .send(|m| {
                    m.embed(|embed| command_error_embed(embed, error))
                        .ephemeral(true)
                })
                .await;
        }

        error => {
            let error_text = format!("{:?}", error);
            let error_variant = error_text.splitn(2, "{").next().unwrap_or("");

            tracing::error!("invoking default error handler for {}", error_variant);

            if let Err(e) = poise::builtins::on_error(error).await {
                tracing::error!(%e, "error in default error handler")
            }
        }
    }
}
