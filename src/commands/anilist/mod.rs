mod embeds;

use crate::{
    anilist::embeds::{anime_embed_builder, manga_embed_builder},
    api::anilist,
    core::{context::CommandContext, menu::Menu},
};
use anyhow::Result;

/// Search for anime on AniList
#[tracing::instrument]
#[poise::command(slash_command)]
pub async fn anime(
    ctx: CommandContext<'_>,
    #[description = "Name of the anime to search for"] name: String,
) -> Result<()> {
    let anime = anilist::find_anime(&reqwest::Client::new(), &name).await?;

    tracing::info!("found {} anime", anime.len());

    Menu::from(anime, anime_embed_builder).send(ctx).await?;

    tracing::info!("spawned anime menu");

    Ok(())
}

/// Search for manga on AniList
#[tracing::instrument]
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
