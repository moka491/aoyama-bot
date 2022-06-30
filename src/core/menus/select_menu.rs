use std::time::Duration;

use poise::serenity_prelude::{
    ButtonStyle, CollectComponentInteraction, CreateComponents, CreateEmbed, CreateSelectMenuOption,
};

use crate::core::{context::CommandContext, interactions::ComponentInteractionExt};
use anyhow::Result;

use crate::core::embeds::EmbedBuilder;

pub type SelectItemMapper<T> = fn(&T) -> String;

pub struct SelectMenu<T>
where
    T: Clone,
{
    data: Vec<T>,
    builder: EmbedBuilder<T>,
    select_item_mapper: SelectItemMapper<T>,
    cur_page_index: i8,
}

impl<T> SelectMenu<T>
where
    T: Clone,
{
    pub fn from(
        mut data: Vec<T>,
        builder: EmbedBuilder<T>,
        select_item_mapper: SelectItemMapper<T>,
    ) -> Self {
        data.truncate(25);

        SelectMenu {
            data,
            builder,
            select_item_mapper,
            cur_page_index: 0,
        }
    }

    fn build_embed<'a>(&self, e: &'a mut CreateEmbed) -> &'a mut CreateEmbed {
        let page_data = self.data[self.cur_page_index as usize].clone();

        (self.builder)(page_data, e)
    }

    fn build_action_row<'a>(&self, c: &'a mut CreateComponents) -> &'a mut CreateComponents {
        c.create_action_row(|ar| {
            ar.create_select_menu(|s| {
                s.custom_id("select_menu").options(|o| {
                    let titles: Vec<String> = self
                        .data
                        .iter()
                        .map(|item| (self.select_item_mapper)(item))
                        .collect();

                    let options: Vec<CreateSelectMenuOption> = titles
                        .into_iter()
                        .enumerate()
                        .map(|(i, title)| {
                            let mut opt: CreateSelectMenuOption = CreateSelectMenuOption::default();
                            opt.label(title).value(i);
                            opt
                        })
                        .collect();

                    o.set_options(options)
                })
            })
        })
        .create_action_row(|ar| {
            ar.create_button(|b| {
                b.style(ButtonStyle::Primary)
                    .label("Confirm")
                    .custom_id("button_confirm")
            })
        })
    }

    pub async fn send(&mut self, ctx: CommandContext<'_>) -> Result<()> {
        let mut msg = ctx
            .send(|m| {
                m.embed(|e| self.build_embed(e))
                    .components(|c| self.build_action_row(c))
            })
            .await?
            .message()
            .await?;

        while let Some(mci) = CollectComponentInteraction::new(ctx.discord())
            .message_id(msg.id)
            .timeout(Duration::from_secs(60))
            .await
        {
            if mci.user.id != ctx.author().id {
                mci.respond_error(
                    &ctx,
                    String::from("Excuse me, but I think you can't do that"),
                )
                .await?;
            }

            match mci.data.custom_id.as_str() {
                "select_menu" => {
                    let index: i8 = mci.data.values.get(0).unwrap().parse().unwrap();
                    self.cur_page_index = index;

                    msg.edit(ctx.discord(), |m| m.embed(|e| self.build_embed(e)))
                        .await?;
                }
                "button_confirm" => break,
                _ => (),
            }

            mci.respond_deferred(&ctx).await?;
        }

        // Remove action buttons on "Done" click or after timeout
        msg.edit(ctx.discord(), |m| {
            m.components(|c| c.set_action_rows(vec![]))
        })
        .await?;

        Ok(())
    }
}
