use anyhow::Result;

use crate::{
    api::donmai::{self, DonmaiImage},
    core::context::CommandContext,
};

#[tracing::instrument(skip(ctx))]
#[poise::command(slash_command, category = "Anime & Manga")]
pub async fn image(
    ctx: CommandContext<'_>,
    #[description = "Optionally add some tags to refine your search"] tags: String,
) -> Result<()> {
    let image: DonmaiImage = donmai::get_random_picture(&ctx.data().client, tags).await?;

    let _ = ctx
        .send(|m| {
            m.embed(|e| {
                e.image(image.file_url)
                    .title("Result")
                    .url(format!("https://donmai.moe/posts/{}", image.id))
            })
        })
        .await?;

    Ok(())
}
