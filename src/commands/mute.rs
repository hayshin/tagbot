use crate::commands::{BotResponseExt, CommandContext, Tag};

pub async fn handle_mute(ctx: CommandContext, arg: String) -> anyhow::Result<()> {
    toggle_mute(ctx, arg, true).await
}

pub async fn handle_unmute(ctx: CommandContext, arg: String) -> anyhow::Result<()> {
    toggle_mute(ctx, arg, false).await
}

async fn toggle_mute(ctx: CommandContext, arg: String, should_mute: bool) -> anyhow::Result<()> {
    let trimmed = arg.trim();
    if trimmed.is_empty() {
        ctx.bot
            .send_error_msg(ctx.msg.chat.id, "Please specify a tag name to mute/unmute.")
            .await?;
        return Ok(());
    }

    let tag = Tag::new(trimmed.to_string());
    let mute_type = tag.to_string();

    let result = if should_mute {
        ctx.db
            .mute_user(ctx.msg.chat.id.0, ctx.user.id, mute_type.clone())
            .await?
    } else {
        ctx.db
            .unmute_user(ctx.msg.chat.id.0, ctx.user.id, mute_type.clone())
            .await?
    };

    if result {
        let response = if should_mute {
            format!(
                "You have been muted for the '{}' tag and won't be mentioned by it",
                tag.as_ref()
            )
        } else {
            format!(
                "You have been unmuted for the '{}' tag and can be mentioned by it again",
                tag.as_ref()
            )
        };
        ctx.bot.send_success_msg(ctx.msg.chat.id, &response).await?;
    } else {
        let response = if should_mute {
            format!("You are already muted for tag '{}'", tag.as_ref())
        } else {
            format!("You were not muted for tag '{}' anyway", tag.as_ref())
        };
        ctx.bot.send_error_msg(ctx.msg.chat.id, &response).await?;
    }

    Ok(())
}
