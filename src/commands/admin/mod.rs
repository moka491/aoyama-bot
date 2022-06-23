use anyhow::Result;

use crate::core::context::CommandContext;

#[tracing::instrument]
#[poise::command(prefix_command, owners_only)]
pub async fn register(ctx: CommandContext<'_>) -> Result<()> {
    poise::builtins::register_application_commands_buttons(ctx).await?;

    tracing::info!("spawned builtin application command buttons");
    Ok(())
}
