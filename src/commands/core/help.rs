use crate::core::context::CommandContext;

use anyhow::Result;

/// Look at all available commands and options
///
/// It appears to me that you are already aware of
/// how to use this command~
///
/// **Usage**
/// `/help`
/// `/help anime`
#[poise::command(slash_command, category = "Bot")]
pub async fn help(
    ctx: CommandContext<'_>,
    #[description = "Specific command to show help about"] command: Option<String>,
) -> Result<()> {
    let config = poise::builtins::HelpConfiguration {
        extra_text_at_bottom: "Use /help <command name> to get more info on a specific command!",
        ..Default::default()
    };
    poise::builtins::help(ctx, command.as_deref(), config).await?;
    Ok(())
}
