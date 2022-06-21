mod embeds;

use crate::anilist::embeds::anime_embed_builder;
use crate::{core::menu::Menu, Context};
use anyhow::Result;

use crate::api::anilist;

/// Search for an anime on AniList
#[poise::command(slash_command)]
pub async fn anime(
    ctx: Context<'_>,
    #[description = "Name of the anime to search for"] name: String,
) -> Result<()> {
    let anime = anilist::find_anime(&reqwest::Client::new(), &name).await?;

    Menu::from(anime, anime_embed_builder).send(ctx).await?;

    Ok(())
}
