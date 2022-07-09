use anyhow::Result;

use crate::{
    api::donmai::{self, DonmaiImage},
    core::context::CommandContext,
};


/// Get a random image from donmai.moe
/// 
/// This command retrieves a random image from the donmai.moe imageboard.
/// You can specify up to two tags to search for, in the typical image booru fashion.
/// Use \*\* around your search term if you don't know the exact tag to search for
/// 
/// **Usage**
/// /image
/// /image *yuru_yuri*
#[tracing::instrument(skip(ctx))]
#[poise::command(slash_command, category = "Anime & Manga")]
pub async fn image(
    ctx: CommandContext<'_>,
    #[description = "Optionally add some tags to refine your search"] tags: Option<String>,
) -> Result<()> {
    
    let image: DonmaiImage = donmai::get_random_picture(&ctx.data().client, tags).await?;

    let _ = ctx
        .send(|m| {
            m.embed(|e| {
                e.image(image.large_file_url)
                    .title("Result")
                    .url(format!("https://donmai.moe/posts/{}", image.id))
            })
        })
        .await?;

    Ok(())
}
