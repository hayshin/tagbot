use teloxide::prelude::*;
use teloxide::utils::markdown;
use crate::commands::{Tag, CommandContext};

pub async fn handle_join(ctx: CommandContext, tag: Tag) -> anyhow::Result<()> {
    if ctx.db.join_tag(ctx.msg.chat.id.0, tag.as_ref().to_string(), &ctx.user).await? {
        ctx.bot.send_message(
            ctx.msg.chat.id,
            format!("You have been added to tag '{}'", markdown::escape(tag.as_ref())),
        ).await?;
    } else {
        ctx.bot.send_message(
            ctx.msg.chat.id,
            format!("You are already in tag '{}'", markdown::escape(tag.as_ref())),
        ).await?;
    }
    Ok(())
}
