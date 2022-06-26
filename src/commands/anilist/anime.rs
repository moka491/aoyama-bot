use crate::{
    api::anilist,
    commands::anilist::embeds::anime_embed_builder,
    core::{context::CommandContext, menu::Menu},
};
use anyhow::Result;

/// Search for anime on AniList
///
/// Look up information about a anime
/// and show the results in discord!
/// All anime matching your search term will be presented to you,
/// so pick the one you'd like to show using the ◀️ ▶️ buttons,
/// and confirm using ⏹️.
///
/// **Usage**
/// `/anime Gochiusa`
/// `/anime Sora Yori`
#[tracing::instrument(skip(ctx))]
#[poise::command(slash_command, category = "Anime & Manga")]
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
