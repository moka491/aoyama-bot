use crate::{
    api::anilist,
    commands::anilist::embeds::anime_embed_builder,
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
