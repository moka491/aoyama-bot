use crate::{
    api::anilist,
    core::{
        context::CommandContext, embeds::anime::manga_embed_builder, menus::select_menu::SelectMenu,
    },
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
    let manga = anilist::find_manga(&ctx.data().client, &name).await?;

    tracing::info!("found {} manga", manga.len());

    SelectMenu::from(manga, manga_embed_builder, |manga| {
        manga.title.romaji.clone().unwrap_or("".into())
    })
    .send(ctx)
    .await?;

    tracing::info!("spawned manga menu");

    Ok(())
}
