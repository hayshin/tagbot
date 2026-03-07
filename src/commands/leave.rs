use teloxide::prelude::*;
use teloxide::utils::markdown;
use std::sync::Arc;
use crate::db::Database;
use crate::commands::normalize_tag;

pub async fn handle_leave(bot: Bot, msg: Message, db: Arc<Database>, tag_name: String) -> anyhow::Result<()> {
    if let Some(user) = &msg.from {
        let tag_name = normalize_tag(tag_name);
        if db.leave_tag(msg.chat.id.0, tag_name.clone(), user.id.0).await? {
            bot.send_message(
                msg.chat.id,
                format!("You have been removed from tag '{}'", markdown::escape(&tag_name)),
            ).await?;
        } else {
            bot.send_message(
                msg.chat.id,
                format!("You are not in tag '{}' or it doesn't exist", markdown::escape(&tag_name)),
            ).await?;
        }
    }
    Ok(())
}
