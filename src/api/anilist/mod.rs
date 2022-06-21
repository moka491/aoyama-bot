pub mod model;
mod queries;
mod util;

use anyhow::Result;
use reqwest::Client;
use std::collections::HashMap;

use crate::api::anilist::{
    model::Anime,
    queries::{GetAnimeResponse, GET_ANIME},
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
