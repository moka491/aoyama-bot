use std::cmp::Reverse;

use crate::CommandContext;
use anyhow::{Context, Result};
use poise::serenity_prelude::Mentionable;

/// Mention everyone who joined your thread
#[poise::command(slash_command)]
pub async fn ping(ctx: CommandContext<'_>) -> Result<()> {
    let channel_id = ctx.channel_id();

    let mut thread_members = channel_id
        .get_thread_members(&ctx.discord().http)
        .await
        .context("It seems like you are not in a thread, dear~")?;

    thread_members.sort_by_key(|member| Reverse(member.join_timestamp.timestamp()));

    let oldest_member = thread_members
        .pop()
        .context("It seems like there was no user here..? I'm confused")?;

    if oldest_member
        .user_id
        .and_then(|id| (id == ctx.author().id).into())
        .is_some()
    {
        let mentions: Vec<String> = thread_members
            .into_iter()
            .filter_map(|member| member.user_id.and_then(|id| Some(id.mention().to_string())))
            .collect();

        ctx.send(|m| {
            m.content(format!(
                "Calling forth everyone in this lovely thread!\n{}",
                mentions.join(" ")
            ))
        })
        .await?;
    }

    Ok(())
}
