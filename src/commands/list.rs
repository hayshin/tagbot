use teloxide::prelude::*;
use teloxide::utils::markdown;
use std::sync::Arc;
use crate::db::Database;

pub async fn handle_list(bot: Bot, msg: Message, db: Arc<Database>) -> anyhow::Result<()> {
    let tags = db.list_tags(msg.chat.id.0).await?;
    let muted_count = db.get_muted_count(msg.chat.id.0).await?;

    if tags.is_empty() {
        bot.send_message(msg.chat.id, format!("No tags exist in this group yet.
Muted users: {}", muted_count)).await?;
    } else {
        let mut tag_list = Vec::new();
        for (tag_name, count) in tags {
            tag_list.push(format!("• {} ({} users)", markdown::escape(&tag_name), count));
        }

        let message = format!("Tags in this group:
{}

Muted users: {}", tag_list.join("
"), muted_count);
        bot.send_message(msg.chat.id, message).await?;
    }
    Ok(())
}
