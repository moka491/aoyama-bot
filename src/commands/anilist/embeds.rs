use poise::serenity_prelude::{Color, CreateEmbed};

use crate::api::anilist::model::Anime;

fn parse_color(mut hex_color: String) -> Option<Color> {
    hex_color.retain(|c| c != '#');
    i32::from_str_radix(&hex_color, 16).map(Color::from).ok()
}

pub fn anime_embed_builder(anime: Anime, e: &mut CreateEmbed) -> &mut CreateEmbed {
    if let Some(romaji_title) = anime.title.romaji {
        e.title(romaji_title).url(anime.site_url);
    }

    if let Some(description) = anime.description {
        e.description(description.replace("<br>", "\n"));
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
