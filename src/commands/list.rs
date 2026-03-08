use crate::commands::CommandContext;
use teloxide::prelude::*;
use teloxide::utils::markdown;

pub async fn handle_list(ctx: CommandContext) -> anyhow::Result<()> {
    let tags = ctx.db.list_tags(ctx.msg.chat.id.0).await?;

    if tags.is_empty() {
        ctx.bot
            .send_message(ctx.msg.chat.id, "No tags exist in this group yet.")
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
{}",
            tag_list.join(
                "
"
            )
        );
        ctx.bot
            .send_message(ctx.msg.chat.id, message)
            .parse_mode(teloxide::types::ParseMode::MarkdownV2)
            .await?;
    }
    Ok(())
}
