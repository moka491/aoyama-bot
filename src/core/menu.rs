use std::time::Duration;

use poise::serenity_prelude::{
    ButtonStyle, CollectComponentInteraction, CreateComponents, CreateEmbed,
    InteractionResponseType,
};

use crate::core::context::CommandContext;
use anyhow::Result;

pub type EmbedBuilder<T> = fn(T, &mut CreateEmbed) -> &mut CreateEmbed;

pub struct Menu<T>
where
    T: Clone,
{
    data: Vec<T>,
    cur_page_index: i8,
    builder: EmbedBuilder<T>,
}

impl<T> Menu<T>
where
    T: Clone,
{
    pub fn from(data: Vec<T>, builder: EmbedBuilder<T>) -> Self {
        Menu {
            data,
            builder,
            cur_page_index: 0,
        }
    }

    fn build_embed<'a>(&self, e: &'a mut CreateEmbed) -> &'a mut CreateEmbed {
        let page_data = self.data[self.cur_page_index as usize].clone();

        (self.builder)(page_data, e)
    }

    fn build_action_row<'a>(&self, c: &'a mut CreateComponents) -> &'a mut CreateComponents {
        let is_first_page = self.cur_page_index <= 0;
        let is_last_page = self.cur_page_index as usize >= self.data.len() - 1;

        c.create_action_row(|ar| {
            ar.create_button(|b| {
                b.style(ButtonStyle::Primary)
                    .label('◀')
                    .custom_id("button_prev")
                    .disabled(is_first_page)
            })
            .create_button(|b| {
                b.style(ButtonStyle::Primary)
                    .label('▶')
                    .custom_id("button_next")
                    .disabled(is_last_page)
            })
            .create_button(|b| {
                b.style(ButtonStyle::Primary)
                    .label('⏹')
                    .custom_id("button_done")
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
            .author_id(ctx.author().id)
            .message_id(msg.id)
            .timeout(Duration::from_secs(60))
            .await
        {
            match mci.data.custom_id.as_str() {
                "button_prev" => self.cur_page_index -= 1,
                "button_next" => self.cur_page_index += 1,
                "button_done" => break,
                _ => (),
            }

            msg.edit(ctx.discord(), |m| {
                m.embed(|e| self.build_embed(e))
                    .components(|c| self.build_action_row(c))
            })
            .await?;

            mci.create_interaction_response(ctx.discord(), |ir| {
                ir.kind(InteractionResponseType::DeferredUpdateMessage)
            })
            .await?;
        }

        // Remove action buttons on "Done" click or after timeout
        msg.edit(ctx.discord(), |m| {
            m.components(|c| c.set_action_rows(vec![]))
        })
        .await?;

        Ok(())
    }
}
