use anyhow::Result;

use crate::core::context::CommandContext;

#[tracing::instrument(skip(ctx))]
#[poise::command(prefix_command, owners_only, hide_in_help)]
pub async fn register_commands(ctx: CommandContext<'_>) -> Result<()> {
    poise::builtins::register_application_commands_buttons(ctx).await?;

    tracing::info!("spawned builtin application command buttons");
    Ok(())
}
