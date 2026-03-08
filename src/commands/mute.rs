use crate::commands::{CommandContext, BotResponseExt};

pub async fn handle_mute(ctx: CommandContext, arg: String) -> anyhow::Result<()> {
    toggle_mute(ctx, arg, true).await
}

pub async fn handle_unmute(ctx: CommandContext, arg: String) -> anyhow::Result<()> {
    toggle_mute(ctx, arg, false).await
}

async fn toggle_mute(ctx: CommandContext, arg: String, should_mute: bool) -> anyhow::Result<()> {
    let mute_type = if arg.trim().to_lowercase() == "ask" {
        "ask".to_string()
    } else {
        "all".to_string()
    };

    let result = if should_mute {
        ctx.db.mute_user(ctx.msg.chat.id.0, ctx.user.id, mute_type.clone()).await?
    } else {
        ctx.db.unmute_user(ctx.msg.chat.id.0, ctx.user.id, mute_type.clone()).await?
    };

    if result {
        let response = match (should_mute, mute_type.as_str()) {
            (true, "ask") => "You have been muted for the 'ask' command and won't be mentioned by it",
            (true, _) => "You have been muted and won't be called in group mentions",
            (false, "ask") => "You have been unmuted for the 'ask' command and can be mentioned by it again",
            (false, _) => "You have been unmuted and will be called in group mentions again",
        };
        ctx.bot.send_success_msg(ctx.msg.chat.id, response).await?;
    } else {
        let response = if should_mute {
            "You are already muted for this"
        } else {
            "You were not muted for this anyway"
        };
        ctx.bot.send_error_msg(ctx.msg.chat.id, response).await?;
    }

    Ok(())
}
