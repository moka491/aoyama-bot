use poise::serenity_prelude::CreateEmbed;

use crate::api::anilist::model::{Anime, MediaFormat, MediaSeason, MediaStatus, Source};

pub fn anime_embed_builder(anime: Anime, e: &mut CreateEmbed) -> &mut CreateEmbed {
    if let Some(romaji_title) = anime.title.romaji {
        e.title(romaji_title).url(anime.site_url);
    }

    if let Some(description) = anime.description {
        e.description(description.replace("<br>", "\n"));
    }

    let format_str = match anime.format {
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
    };

    e.field("Format", format_str, true);

    if let Some(episodes) = anime.episodes {
        e.field("Episodes", episodes, true);
    };

    let status_str = match anime.status {
        MediaStatus::Finished => "Finished",
        MediaStatus::Releasing => "Releasing",
        MediaStatus::NotYetReleased => "Not yet released",
        MediaStatus::Cancelled => "Cancelled",
        MediaStatus::Hiatus => "On hiatus",
    };

    e.field("Status", status_str, true);

    if let Some(season) = anime.season {
        let season_str = match season {
            MediaSeason::Winter => "Winter",
            MediaSeason::Spring => "Spring",
            MediaSeason::Summer => "Summer",
            MediaSeason::Fall => "Fall",
        };

        let season_year_str: String = anime
            .season_year
            .map(|v| v.to_string())
            .unwrap_or("".into());

        e.field(
            "Season",
            format!("{} {}", season_str, season_year_str),
            true,
        );
    };

    if let Some(score) = anime.average_score {
        e.field("Score (avg)", score, true);
    };

    if let Some(studio) = anime.studios.nodes.get(0) {
        e.field("Studio", &studio.name, true);
    }

    if let Some(source) = anime.source {
        let source_str = match source {
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
        };

        e.field("Source", source_str, true);
    }

    if anime.genres.len() > 0 {
        e.field("Genres", anime.genres.join(", "), true);
    }

    e.thumbnail(anime.cover_image.medium);

    if let Some(color_str) = anime.cover_image.color {
        let len = color_str.len();

        if len >= 6 {
            let trimmed = &color_str[len - 6..];
            if let Ok(color) = i32::from_str_radix(trimmed, 16) {
                e.color(color);
            }
        }
    }

    e
}
