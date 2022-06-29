use std::sync::Arc;

use poise::serenity_prelude::{
    InteractionResponseType, MessageComponentInteraction, SerenityError,
};

use super::context::CommandContext;
use crate::core::error::command_error_embed;

pub async fn mci_acknowledge(
    mci: &Arc<MessageComponentInteraction>,
    ctx: &CommandContext<'_>,
) -> Result<(), SerenityError> {
    mci.create_interaction_response(ctx.discord(), |ir| {
        ir.kind(InteractionResponseType::DeferredUpdateMessage)
    })
    .await
}

// pub async fn mci_respond<'a, F>(
//     mci: &Arc<MessageComponentInteraction>,
//     ctx: &CommandContext<'_>,
//     response_builder: F,
// ) -> Result<(), SerenityError>
// where
//     for<'b> F: FnOnce(
//         &'b mut CreateInteractionResponseData<'a>,
//     ) -> &'b mut CreateInteractionResponseData<'a>,
// {
//     mci.create_interaction_response(ctx.discord(), |r| {
//         r.kind(InteractionResponseType::ChannelMessageWithSource)
//             .interaction_response_data(response_builder)
//     })
//     .await
// }

pub async fn mci_respond_err(
    mci: &Arc<MessageComponentInteraction>,
    ctx: &CommandContext<'_>,
    message: String,
) -> Result<(), SerenityError> {
    mci.create_interaction_response(ctx.discord(), |r| {
        r.kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|d| {
                d.ephemeral(true).embed(|e| command_error_embed(e, message))
            })
    })
    .await
}
