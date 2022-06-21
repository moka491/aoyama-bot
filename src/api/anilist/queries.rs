use super::model::{Anime, Media, Page, Response};

pub const GET_ANIME: &str = r#"
query($name: String) {
    Page(page: 1, perPage: 30) {
        media(type: ANIME, search: $name) {
            id
            title {
                romaji
                english
            }
            description
            format
            episodes
            status(version: 2)
            season
            seasonYear
            averageScore
            studios(isMain:true) {
                nodes {
                    name
                }
            }
            source
            genres
            siteUrl
            coverImage {
                medium
                color
            }
        }
    }
}
"#;

pub type GetAnimeResponse = Response<Page<Media<Anime>>>;
