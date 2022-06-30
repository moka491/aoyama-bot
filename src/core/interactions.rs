use std::{future::Future, sync::Arc};

use async_trait::async_trait;
use poise::serenity_prelude::{
    InteractionResponseType, MessageComponentInteraction, SerenityError,
};

use super::context::CommandContext;
use crate::core::error::command_error_embed;

#[async_trait]
pub trait ComponentInteractionExt {
    async fn respond_deferred(&self, ctx: &CommandContext<'_>) -> Result<(), SerenityError>;

    async fn respond_error(
        &self,
        ctx: &CommandContext<'_>,
        message: String,
    ) -> Result<(), SerenityError>;
}

#[async_trait]
impl ComponentInteractionExt for MessageComponentInteraction {
    async fn respond_deferred(&self, ctx: &CommandContext<'_>) -> Result<(), SerenityError> {
        self.create_interaction_response(ctx.discord(), |ir| {
            ir.kind(InteractionResponseType::DeferredUpdateMessage)
        })
        .await
    }

    async fn respond_error(
        &self,
        ctx: &CommandContext<'_>,
        message: String,
    ) -> Result<(), SerenityError> {
        self.create_interaction_response(ctx.discord(), |r| {
            r.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|d| {
                    d.ephemeral(true).embed(|e| command_error_embed(e, message))
                })
        })
        .await
    }
}
