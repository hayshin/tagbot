use teloxide::prelude::*;
use teloxide::utils::markdown;
use std::sync::Arc;
use crate::db::Database;
use crate::models::UserInfo;
use crate::commands::normalize_tag;

pub async fn handle_join(bot: Bot, msg: Message, db: Arc<Database>, tag_name: String) -> anyhow::Result<()> {
    if let Some(user) = &msg.from {
        let tag_name = normalize_tag(tag_name);
        let user_info = UserInfo {
            id: user.id.0,
            username: user.username.clone(),
            first_name: user.first_name.clone(),
        };

        if db.join_tag(msg.chat.id.0, tag_name.clone(), &user_info).await? {
            bot.send_message(
                msg.chat.id,
                format!("You have been added to tag '{}'", markdown::escape(&tag_name)),
            ).await?;
        } else {
            bot.send_message(
                msg.chat.id,
                format!("You are already in tag '{}'", markdown::escape(&tag_name)),
            ).await?;
        }
    }
    Ok(())
}
