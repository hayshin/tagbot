use teloxide::prelude::*;
use crate::commands::CommandContext;

pub async fn handle_unmute(ctx: CommandContext, arg: String) -> anyhow::Result<()> {
    let mute_type = if arg.trim().to_lowercase() == "ask" {
        "ask".to_string()
    } else {
        "all".to_string()
    };

    if ctx.db.unmute_user(ctx.msg.chat.id.0, ctx.user.id, mute_type.clone()).await? {
        let response = if mute_type == "ask" {
            "You have been unmuted for the 'ask' command and can be mentioned by it again"
        } else {
            "You have been unmuted and will be called in group mentions again"
        };
        ctx.bot.send_message(ctx.msg.chat.id, response).await?;
    } else {
        ctx.bot.send_message(ctx.msg.chat.id, "You were not muted for this anyway").await?;
    }
    Ok(())
}
