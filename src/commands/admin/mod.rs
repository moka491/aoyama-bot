use crate::CommandContext;
use anyhow::Result;
use tracing::instrument;

#[instrument]
#[poise::command(prefix_command, owners_only)]
pub async fn register(ctx: CommandContext<'_>) -> Result<()> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}
