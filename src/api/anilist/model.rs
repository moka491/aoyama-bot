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

impl ToString for MediaFormat {
    fn to_string(&self) -> String {
        match self {
            MediaFormat::Manga => "Manga",
            MediaFormat::Tv => "TV",
            MediaFormat::TvShort => "TV Short",
            MediaFormat::Movie => "Movie",
            MediaFormat::Special => "Special",
            MediaFormat::Ova => "OVA",
            MediaFormat::Ona => "ONA",
            MediaFormat::Music => "Music",
            MediaFormat::Novel => "Novel",
            MediaFormat::OneShot => "One-Shot",
        }
        .into()
    }
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

impl ToString for MediaStatus {
    fn to_string(&self) -> String {
        match self {
            MediaStatus::Finished => "Finished",
            MediaStatus::Releasing => "Releasing",
            MediaStatus::NotYetReleased => "Not yet released",
            MediaStatus::Cancelled => "Cancelled",
            MediaStatus::Hiatus => "On hiatus",
        }
        .into()
    }
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MediaSeason {
    Winter,
    Spring,
    Summer,
    Fall,
}

impl ToString for MediaSeason {
    fn to_string(&self) -> String {
        match self {
            MediaSeason::Winter => "Winter",
            MediaSeason::Spring => "Spring",
            MediaSeason::Summer => "Summer",
            MediaSeason::Fall => "Fall",
        }
        .into()
    }
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

impl ToString for Source {
    fn to_string(&self) -> String {
        match self {
            Source::Original => "Original",
            Source::Manga => "Manga",
            Source::LightNovel => "Light Novel",
            Source::VisualNovel => "Visual Novel",
            Source::VideoGame => "Video Game",
            Source::Other => "Other",
            Source::Novel => "Novel",
            Source::Doujinshi => "Doujinshi",
            Source::Anime => "Anime",
            Source::WebNovel => "Web Novel",
            Source::LiveAction => "Live Action",
            Source::Game => "Game",
            Source::Comic => "Comic",
            Source::MultimediaProject => "Multimedia Project",
            Source::PictureBook => "Picture Book",
        }
        .into()
    }
}

#[derive(Deserialize, Clone)]
pub struct MediaCoverImage {
    pub medium: String,
    pub color: Option<String>,
}
