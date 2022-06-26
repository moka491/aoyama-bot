use crate::{
    api::anilist,
    commands::anilist::embeds::manga_embed_builder,
    core::{context::CommandContext, menu::Menu},
};
use anyhow::Result;

/// Search for manga on AniList
#[tracing::instrument(skip(ctx))]
#[poise::command(slash_command)]
pub async fn manga(
    ctx: CommandContext<'_>,
    #[description = "Name of the manga to search for"] name: String,
) -> Result<()> {
    let manga = anilist::find_manga(&reqwest::Client::new(), &name).await?;

    tracing::info!("found {} manga", manga.len());

    Menu::from(manga, manga_embed_builder).send(ctx).await?;

    tracing::info!("spawned manga menu");

    Ok(())
}
