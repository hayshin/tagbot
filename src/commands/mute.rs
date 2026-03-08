use teloxide::prelude::*;
use std::sync::Arc;
use crate::db::Database;

pub async fn handle_mute(bot: Bot, msg: Message, db: Arc<Database>, arg: String) -> anyhow::Result<()> {
    let mute_type = if arg.trim().to_lowercase() == "ask" {
        "ask".to_string()
    } else {
        "all".to_string()
    };

    if let Some(user) = &msg.from {
        if db.mute_user(msg.chat.id.0, user.id.0, mute_type.clone()).await? {
            let response = if mute_type == "ask" {
                "You have been muted for the 'ask' command and won't be mentioned by it"
            } else {
                "You have been muted and won't be called in group mentions"
            };
            bot.send_message(msg.chat.id, response).await?;
        } else {
            bot.send_message(msg.chat.id, "You are already muted for this").await?;
        }
    }
    Ok(())
}
