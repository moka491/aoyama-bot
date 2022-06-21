use serde::Deserialize;

/* Helper structs for api responses */
#[derive(Deserialize, Clone)]
pub struct Response<T> {
    pub data: T,
}

#[derive(Deserialize, Clone)]
pub struct Page<T> {
    #[serde(rename = "Page")]
    pub page: T,
}

#[derive(Deserialize, Clone)]
pub struct Media<T> {
    pub media: Vec<T>,
}

/* Actually relevant data structs */
#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Anime {
    pub id: u32,
    pub title: MediaTitle,
    pub description: Option<String>,
    pub format: MediaFormat,
    pub episodes: Option<u16>,
    pub status: MediaStatus,
    pub season: Option<MediaSeason>,
    pub season_year: Option<u16>,
    pub average_score: Option<u8>,
    pub studios: StudioConnection,
    pub source: Option<Source>,
    pub genres: Vec<String>,
    pub site_url: String,
    pub cover_image: MediaCoverImage,
}

#[derive(Deserialize, Clone)]
pub struct MediaTitle {
    pub romaji: Option<String>,
    pub english: Option<String>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MediaFormat {
    Tv,
    TvShort,
    Movie,
    Special,
    Ova,
    Ona,
    Music,
    Manga,
    Novel,
    OneShot,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MediaStatus {
    Finished,
    Releasing,
    NotYetReleased,
    Cancelled,
    Hiatus,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MediaSeason {
    Winter,
    Spring,
    Summer,
    Fall,
}

#[derive(Deserialize, Clone)]
pub struct StudioConnection {
    pub nodes: Vec<Studio>,
}

#[derive(Deserialize, Clone)]
pub struct Studio {
    pub name: String,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Source {
    Original,
    Manga,
    LightNovel,
    VisualNovel,
    VideoGame,
    Other,
    Novel,
    Doujinshi,
    Anime,
    WebNovel,
    LiveAction,
    Game,
    Comic,
    MultimediaProject,
    PictureBook,
}

#[derive(Deserialize, Clone)]
pub struct MediaCoverImage {
    pub medium: String,
    pub color: Option<String>,
}
