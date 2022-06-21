use anyhow::{Context, Result};
use reqwest::{Client, Response};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
struct GraphQLBody<'a> {
    query: &'a str,
    variables: &'a HashMap<&'a str, &'a str>,
}

pub async fn anilist_request(
    client: &Client,
    query: &str,
    variables: &HashMap<&str, &str>,
) -> Result<Response> {
    client
        .post("https://graphql.anilist.co/")
        .json(&GraphQLBody { query, variables })
        .send()
        .await
        .context("Failed to reach anilist api")
}
