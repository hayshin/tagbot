use teloxide::prelude::*;
use teloxide::utils::markdown;
use std::sync::Arc;
use futures::future::join_all;
use crate::db::Database;
use crate::commands::normalize_tag;

pub async fn handle_call(bot: Bot, msg: Message, db: Arc<Database>, tag_name: String) -> anyhow::Result<()> {
    let tag_name = normalize_tag(tag_name);
    let users_to_call = db.get_tag_users(msg.chat.id.0, tag_name.clone()).await?;

    if users_to_call.is_empty() {
        bot.send_message(msg.chat.id, format!("No users in tag '{}' (or they are all muted)", markdown::escape(&tag_name))).await?;
    } else {
        let mentions: Vec<String> = users_to_call
            .iter()
            .map(|user| user.info.mention())
            .collect();

        let message = format!("Calling tag '{}': {}", markdown::escape(&tag_name), mentions.join(" "));

        bot.send_message(msg.chat.id, message)
            .parse_mode(teloxide::types::ParseMode::MarkdownV2)
            .await?;

        // Also send direct messages to users who have started the bot privately
        let from_chat_title = msg.chat.title().unwrap_or("this group");
        let from_name = format!("{} ({})", 
            markdown::escape(&msg.from.as_ref().map(|u| u.first_name.clone()).unwrap_or_else(|| "Someone".to_string())), 
            markdown::escape(from_chat_title)
        );

        let dm_futures = users_to_call.into_iter().filter(|u| u.is_private).map(|user| {
            let bot = bot.clone();
            let tag_name = tag_name.clone();
            let from_name = from_name.clone();
            async move {
                let dm_message = format!("🔔 You were called in {} for tag '{}'!", from_name, tag_name);
                let _ = bot.send_message(teloxide::types::ChatId(user.info.id as i64), dm_message).await;
            }
        });

        join_all(dm_futures).await;
    }
    Ok(())
}
