use teloxide::prelude::*;
use std::sync::Arc;
use crate::db::Database;

pub async fn handle_mute(bot: Bot, msg: Message, db: Arc<Database>) -> anyhow::Result<()> {
    if let Some(user) = &msg.from {
        if db.mute_user(msg.chat.id.0, user.id.0).await? {
            bot.send_message(msg.chat.id, "You have been muted and won't be called in group mentions").await?;
        } else {
            bot.send_message(msg.chat.id, "You are already muted").await?;
        }
    }
    Ok(())
}
