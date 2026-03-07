use teloxide::prelude::*;
use std::sync::Arc;
use crate::db::Database;

pub async fn handle_unmute(bot: Bot, msg: Message, db: Arc<Database>) -> anyhow::Result<()> {
    if let Some(user) = &msg.from {
        if db.unmute_user(msg.chat.id.0, user.id.0).await? {
            bot.send_message(msg.chat.id, "You have been unmuted and will be called in group mentions again").await?;
        } else {
            bot.send_message(msg.chat.id, "You were not muted anyway").await?;
        }
    }
    Ok(())
}
