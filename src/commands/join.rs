use crate::commands::{BotResponseExt, CommandContext, Tag};

pub async fn handle_join(ctx: CommandContext, tag: Tag) -> anyhow::Result<()> {
    if ctx
        .db
        .join_tag(ctx.msg.chat.id.0, tag.as_ref().to_string(), &ctx.user)
        .await?
    {
        ctx.bot
            .send_success_msg(
                ctx.msg.chat.id,
                &format!("Вы были добавлены в тег '{}'", tag.as_ref()),
            )
            .await?;
    } else {
        ctx.bot
            .send_error_msg(
                ctx.msg.chat.id,
                &format!("Вы уже состоите в теге '{}'", tag.as_ref()),
            )
            .await?;
    }
    Ok(())
}
