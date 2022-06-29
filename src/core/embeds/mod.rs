use poise::serenity_prelude::CreateEmbed;

pub mod anime;

pub type EmbedBuilder<T> = fn(T, &mut CreateEmbed) -> &mut CreateEmbed;
