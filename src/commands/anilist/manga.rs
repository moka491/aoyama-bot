use crate::{
    api::anilist,
    core::{context::CommandContext, embeds::anime::manga_embed_builder, menu::Menu},
};
use anyhow::Result;

/// Search for manga on AniList
///
/// Look up information about a manga
/// and show the results in discord!
/// All manga matching your search term will be presented to you,
/// so pick the one you'd like to show using the ◀️ ▶️ buttons,
/// and confirm using ⏹️.
///
/// **Usage**
/// `/manga Gochiusa`
/// `/manga Yuru Yuri`
#[tracing::instrument(skip(ctx))]
#[poise::command(slash_command, category = "Anime & Manga")]
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
