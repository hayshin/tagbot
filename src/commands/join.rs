use teloxide::prelude::*;
use teloxide::utils::markdown;
use crate::commands::{normalize_tag, CommandContext};

pub async fn handle_join(ctx: CommandContext, tag_name: String) -> anyhow::Result<()> {
    let tag_name = normalize_tag(tag_name);

    if ctx.db.join_tag(ctx.msg.chat.id.0, tag_name.clone(), &ctx.user).await? {
        ctx.bot.send_message(
            ctx.msg.chat.id,
            format!("You have been added to tag '{}'", markdown::escape(&tag_name)),
        ).await?;
    } else {
        ctx.bot.send_message(
            ctx.msg.chat.id,
            format!("You are already in tag '{}'", markdown::escape(&tag_name)),
        ).await?;
    }
    Ok(())
}
