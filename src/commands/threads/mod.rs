use crate::core::context::CommandContext;
use anyhow::{Context, Result};
use poise::serenity_prelude::Mentionable;

/// Mention everyone who joined your thread
#[tracing::instrument(skip(ctx))]
#[poise::command(slash_command)]
pub async fn ping(ctx: CommandContext<'_>) -> Result<()> {
    let channel_id = ctx.channel_id();

    let bot_id = ctx.discord().http.get_current_user().await?.id;
    let author_id = ctx.author().id;

    let thread_members = &channel_id
        .get_thread_members(&ctx.discord().http)
        .await
        .context("It seems like you are not in a thread, dear~")?;

    tracing::info!("found {} thread members", thread_members.len());

    let thread_owner = thread_members
        .iter()
        .min_by_key(|member| member.join_timestamp.timestamp())
        .context("It seems like there was no user here..? I'm confused")?;

    tracing::info!(?thread_owner, "found thread owner");

    let owner_id = thread_owner
        .user_id
        .and_then(|id| if id == author_id { Some(id) } else { None })
        .context("Ara, you shouldn't be doing this without permission~")?;

    let mentions: Vec<String> = thread_members
        .into_iter()
        .filter_map(|member| member.user_id)
        .filter(|id| *id != owner_id && *id != bot_id)
        .map(|id| id.mention().to_string())
        .collect();

    tracing::info!("prepared {} mentions", mentions.len());

    ctx.send(|m| {
        m.content(format!(
            "Calling forth everyone in this lovely thread!\n{}",
            mentions.join(" ")
        ))
    })
    .await?;

    tracing::info!("sent response");

    Ok(())
}
