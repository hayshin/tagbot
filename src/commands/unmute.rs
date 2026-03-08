use teloxide::prelude::*;
use std::sync::Arc;
use crate::db::Database;

pub async fn handle_unmute(bot: Bot, msg: Message, db: Arc<Database>, arg: String) -> anyhow::Result<()> {
    let mute_type = if arg.trim().to_lowercase() == "ask" {
        "ask".to_string()
    } else {
        "all".to_string()
    };

    if let Some(user) = &msg.from {
        if db.unmute_user(msg.chat.id.0, user.id.0, mute_type.clone()).await? {
            let response = if mute_type == "ask" {
                "You have been unmuted for the 'ask' command and can be mentioned by it again"
            } else {
                "You have been unmuted and will be called in group mentions again"
            };
            bot.send_message(msg.chat.id, response).await?;
        } else {
            bot.send_message(msg.chat.id, "You were not muted for this anyway").await?;
        }
    }
    Ok(())
}
