use teloxide::prelude::*;
use crate::commands::CommandContext;

pub async fn handle_mute(ctx: CommandContext, arg: String) -> anyhow::Result<()> {
    let mute_type = if arg.trim().to_lowercase() == "ask" {
        "ask".to_string()
    } else {
        "all".to_string()
    };

    if ctx.db.mute_user(ctx.msg.chat.id.0, ctx.user.id, mute_type.clone()).await? {
        let response = if mute_type == "ask" {
            "You have been muted for the 'ask' command and won't be mentioned by it"
        } else {
            "You have been muted and won't be called in group mentions"
        };
        ctx.bot.send_message(ctx.msg.chat.id, response).await?;
    } else {
        ctx.bot.send_message(ctx.msg.chat.id, "You are already muted for this").await?;
    }
    Ok(())
}
