use std::time::Duration;

use poise::serenity_prelude::{
    ButtonStyle, CollectComponentInteraction, CreateComponents, CreateEmbed, CreateSelectMenuOption,
};

use crate::core::{
    context::CommandContext, embeds::EmbedBuilder, interactions::ComponentInteractionExt,
    responses::Response,
};
use anyhow::Result;

pub type ItemTitleMapper<T> = fn(&T) -> String;

fn truncate(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx],
    }
}

pub struct SelectMenu<T>
where
    T: Clone,
{
    data: Vec<T>,
    dropdown_labels: Vec<String>,
    builder: EmbedBuilder<T>,
    cur_page_index: i8,
}

impl<T> SelectMenu<T>
where
    T: Clone,
{
    pub fn from(
        mut data: Vec<T>,
        builder: EmbedBuilder<T>,
        label_mapper: ItemTitleMapper<T>,
    ) -> Self {
        // truncate data to 25 items (discord select menu limit)
        data.truncate(25);
        // generate labels and truncate to 100 characters (discord select menu limit)
        let dropdown_labels = data
            .iter()
            .map(label_mapper)
            .map(|s| truncate(&s, 100).into())
            .collect();

        SelectMenu {
            data,
            dropdown_labels,
            builder,
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
                    let options: Vec<CreateSelectMenuOption> = self
                        .dropdown_labels
                        .iter()
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
                mci.respond_error(&ctx, Response::NoPermission.to_string())
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
