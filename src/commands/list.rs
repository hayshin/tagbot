use crate::commands::CommandContext;
use teloxide::prelude::*;
use teloxide::utils::markdown;

pub async fn handle_list(ctx: CommandContext) -> anyhow::Result<()> {
    let tags = ctx.db.list_tags(ctx.msg.chat.id.0).await?;
    let muted_count = ctx.db.get_muted_count(ctx.msg.chat.id.0).await?;

    if tags.is_empty() {
        ctx.bot
            .send_message(
                ctx.msg.chat.id,
                format!(
                    "No tags exist in this group yet.
Muted users: {}",
                    muted_count
                ),
            )
            .await?;
    } else {
        let mut tag_list = Vec::new();
        for (tag_name, count) in tags {
            tag_list.push(format!(
                "• {} ({} users)",
                markdown::escape(&tag_name),
                count
            ));
        }

        let message = format!(
            "Tags in this group:
{}

Muted users: {}",
            tag_list.join(
                "
"
            ),
            muted_count
        );
        ctx.bot.send_message(ctx.msg.chat.id, message).await?;
    }
    Ok(())
}
