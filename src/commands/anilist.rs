use crate::Context;
use anyhow::Result;

use crate::api::anilist;

/// Search for an anime on AniList
#[poise::command(slash_command)]
pub async fn anime(
    ctx: Context<'_>,
    #[description = "Name of the anime to search for"] name: String,
) -> Result<()> {
    let anime = anilist::find_anime(&reqwest::Client::new(), &name).await?;

    let names: Vec<String> = anime
        .into_iter()
        .filter_map(|anime| anime.title.romaji)
        .collect();

    ctx.say(format!(
        "found the following anime: \n {}",
        names.join("\n")
    ))
    .await?;

    Ok(())
}
