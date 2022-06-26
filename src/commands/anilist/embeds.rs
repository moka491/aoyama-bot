use crate::api::anilist::model::{Anime, Manga};
use poise::serenity_prelude::{Color, CreateEmbed};

fn parse_color(mut hex_color: String) -> Option<Color> {
    hex_color.retain(|c| c != '#');
    i32::from_str_radix(&hex_color, 16).map(Color::from).ok()
}

fn strip_text(text: String) -> String {
    let plain = text.replace("<br>", "\n");

    plain.lines().take(1).collect()
}

pub fn anime_embed_builder(anime: Anime, e: &mut CreateEmbed) -> &mut CreateEmbed {
    if let Some(romaji_title) = anime.title.romaji {
        e.title(romaji_title).url(anime.site_url);
    }

    if let Some(description) = anime.description {
        e.description(strip_text(description));
    }

    e.field("Format", anime.format, true);

    if let Some(episodes) = anime.episodes {
        e.field("Episodes", episodes, true);
    };

    e.field("Status", anime.status, true);

    if let Some(season) = anime.season {
        let season_year_str: String = anime
            .season_year
            .map(|v| v.to_string())
            .unwrap_or("".into());

        e.field(
            "Season",
            format!("{} {}", season.to_string(), season_year_str),
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
        e.field("Source", source.to_string(), true);
    }

    if anime.genres.len() > 0 {
        e.field("Genres", anime.genres.join(", "), true);
    }

    e.thumbnail(anime.cover_image.medium);

    if let Some(color) = anime.cover_image.color.and_then(parse_color) {
        e.color(color);
    }

    e
}

pub fn manga_embed_builder(manga: Manga, e: &mut CreateEmbed) -> &mut CreateEmbed {
    if let Some(romaji_title) = manga.title.romaji {
        e.title(romaji_title).url(manga.site_url);
    }

    if let Some(description) = manga.description {
        e.description(strip_text(description));
    }

    e.field("Format", manga.format, true);

    if let Some(chapters) = manga.chapters {
        e.field("Chapters", chapters, true);
    };

    if let Some(volumes) = manga.volumes {
        e.field("Volumes", volumes, true);
    };

    e.field("Status", manga.status, true);

    if let Some(score) = manga.average_score {
        e.field("Score (avg)", score, true);
    };

    if let Some(studio) = manga.studios.nodes.get(0) {
        e.field("Studio", &studio.name, true);
    }

    if let Some(source) = manga.source {
        e.field("Source", source.to_string(), true);
    }

    if manga.genres.len() > 0 {
        e.field("Genres", manga.genres.join(", "), true);
    }

    e.thumbnail(manga.cover_image.medium);

    if let Some(color) = manga.cover_image.color.and_then(parse_color) {
        e.color(color);
    }

    e
}
