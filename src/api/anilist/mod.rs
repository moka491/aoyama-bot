pub mod model;
mod queries;
mod util;

use anyhow::Result;
use reqwest::Client;
use std::collections::HashMap;

use crate::api::anilist::{
    model::{Anime, Manga},
    queries::{GetAnimeResponse, GetMangaResponse, GET_ANIME, GET_MANGA},
    util::anilist_request,
};

pub async fn find_anime(client: &Client, name: &str) -> Result<Vec<Anime>> {
    let variables = HashMap::from([("name", name)]);

    let resp: GetAnimeResponse = anilist_request(client, GET_ANIME, &variables)
        .await?
        .json()
        .await?;

    let anime: Vec<Anime> = resp.data.page.media;

    Ok(anime)
}

pub async fn find_manga(client: &Client, name: &str) -> Result<Vec<Manga>> {
    let variables = HashMap::from([("name", name)]);

    let resp: GetMangaResponse = anilist_request(client, GET_MANGA, &variables)
        .await?
        .json()
        .await?;

    let manga: Vec<Manga> = resp.data.page.media;

    Ok(manga)
}
