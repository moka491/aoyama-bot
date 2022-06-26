use crate::core::context::CommandContext;

use anyhow::Result;

/// Look at all available commands and options
#[poise::command(slash_command)]
pub async fn help(
    ctx: CommandContext<'_>,
    #[description = "Specific command to show help about"] command: Option<String>,
) -> Result<()> {
    let config = poise::builtins::HelpConfiguration {
        ..Default::default()
    };
    poise::builtins::help(ctx, command.as_deref(), config).await?;
    Ok(())
}
