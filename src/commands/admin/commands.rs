use crate::Context;
use anyhow::Result;

#[poise::command(prefix_command, owners_only)]
pub async fn register(ctx: Context<'_>) -> Result<()> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}
