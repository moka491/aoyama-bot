use crate::core::{context::CommandContext, responses::Response};
use anyhow::{Context, Result};
use poise::serenity_prelude::{Mentionable, ThreadMember};

/// Summon everyone in your thread
///
/// If you are the owner of a thread, you
/// can use this command to ping everyone else
/// who is currently in the thread.
/// This can be very useful to make an announcement
/// or to summon everyone for an event.
/// Naturally, this can only be used every 120 seconds, to prevent spam :)
///
/// **Usage**
/// `/summon` (in a thread you are the owner of)
#[tracing::instrument(skip(ctx))]
#[poise::command(slash_command, category = "Utilities")]
pub async fn summon(ctx: CommandContext<'_>) -> Result<()> {
    let channel_id = ctx.channel_id();

    let bot_id = ctx.discord().http.get_current_user().await?.id;
    let author_id = ctx.author().id;

    let thread_members = &channel_id
        .get_thread_members(&ctx.discord().http)
        .await
        .context(Response::NotInThread.to_string())?;

    tracing::info!(?thread_members, "found {} thread members", thread_members.len());

    let thread_owner: &ThreadMember = thread_members
        .iter()
        .min_by_key(|member| member.join_timestamp.timestamp_millis())
        .context(Response::NoUsersInThread.to_string())?;

    tracing::info!(?thread_owner, "found thread owner");

    let owner_id = thread_owner
        .user_id
        .and_then(|id| if id == author_id { Some(id) } else { None })
        .context(Response::NoPermission.to_string())?;

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
