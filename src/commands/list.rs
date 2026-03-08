use crate::commands::CommandContext;
use teloxide::prelude::*;
use teloxide::utils::html;

pub async fn handle_list(ctx: CommandContext) -> anyhow::Result<()> {
    let tags = ctx.db.list_tags(ctx.msg.chat.id.0).await?;

    if tags.is_empty() {
        ctx.bot
            .send_message(ctx.msg.chat.id, "В этой группе пока нет тегов.")
            .await?;
    } else {
        let mut tag_list = Vec::new();
        for (tag_name, count) in tags {
            tag_list.push(format!(
                "• {} ({} уч.)",
                html::escape(&tag_name),
                count
            ));
        }

        let message = format!(
            "Теги в этой группе:
{}",
            tag_list.join(
                "
"
            )
        );
        ctx.bot
            .send_message(ctx.msg.chat.id, message)
            .parse_mode(teloxide::types::ParseMode::Html)
            .await?;
    }
    Ok(())
}
