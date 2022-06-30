use crate::{
    api::anilist,
    core::{
        context::CommandContext, embeds::anime::anime_embed_builder, menus::select_menu::SelectMenu,
    },
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

    SelectMenu::from(anime, anime_embed_builder, |anime| {
        anime.title.romaji.clone().unwrap_or("".into())
    })
    .send(ctx)
    .await?;

    tracing::info!("spawned anime menu");

    Ok(())
}
