use teloxide::prelude::*;
use teloxide::utils::markdown;
use crate::commands::{normalize_tag, CommandContext};

pub async fn handle_leave(ctx: CommandContext, tag_name: String) -> anyhow::Result<()> {
    let tag_name = normalize_tag(tag_name);
    if ctx.db.leave_tag(ctx.msg.chat.id.0, tag_name.clone(), ctx.user.id).await? {
        ctx.bot.send_message(
            ctx.msg.chat.id,
            format!("You have been removed from tag '{}'", markdown::escape(&tag_name)),
        ).await?;
    } else {
        ctx.bot.send_message(
            ctx.msg.chat.id,
            format!("You are not in tag '{}' or it doesn't exist", markdown::escape(&tag_name)),
        ).await?;
    }
    Ok(())
}
