use crate::commands::{BotResponseExt, CommandContext, Tag};

pub async fn handle_leave(ctx: CommandContext, tag: Tag) -> anyhow::Result<()> {
    if ctx
        .db
        .leave_tag(ctx.msg.chat.id.0, tag.as_ref().to_string(), ctx.user.id)
        .await?
    {
        ctx.bot
            .send_success_msg(
                ctx.msg.chat.id,
                &format!("Вы вышли из тега '{}'", tag.as_ref()),
            )
            .await?;
    } else {
        ctx.bot
            .send_error_msg(
                ctx.msg.chat.id,
                &format!(
                    "Вы не состоите в теге '{}' или его не существует",
                    tag.as_ref()
                ),
            )
            .await?;
    }
    Ok(())
}
