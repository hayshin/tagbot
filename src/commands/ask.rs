use teloxide::prelude::*;
use teloxide::utils::markdown;
use std::sync::Arc;
use rand::seq::SliceRandom;
use crate::db::Database;
use crate::commands::normalize_tag;

pub async fn handle_ask(bot: Bot, msg: Message, db: Arc<Database>, input: String) -> anyhow::Result<()> {
    let input = input.trim();
    if input.is_empty() {
        return pick_and_respond(bot, msg, db, "all".to_string(), "").await;
    }

    let mut parts = input.splitn(2, ' ');
    let first_word = parts.next().unwrap_or("");
    let rest = parts.next().unwrap_or("");

    let normalized_first = normalize_tag(first_word.to_string());
    
    // Check if the first word is a known tag
    if db.tag_exists(msg.chat.id.0, normalized_first.clone()).await? {
        pick_and_respond(bot, msg, db, normalized_first, rest).await
    } else {
        // Not a tag, so the whole input is the question for the "all" tag
        pick_and_respond(bot, msg, db, "all".to_string(), input).await
    }
}

async fn pick_and_respond(bot: Bot, msg: Message, db: Arc<Database>, tag_name: String, question: &str) -> anyhow::Result<()> {
    let users = db.get_tag_users(msg.chat.id.0, tag_name.clone()).await?;

    if users.is_empty() {
        bot.send_message(msg.chat.id, format!("No users in tag '{}'", markdown::escape(&tag_name))).await?;
    } else {
        let picked_user = users.choose(&mut rand::thread_rng()).unwrap();
        let mention = picked_user.info.mention();

        let response = if question.is_empty() {
            format!("The chosen one from tag '{}' is {}!", markdown::escape(&tag_name), mention)
        } else {
            format!(r"{} \- it's {}!", markdown::escape(question), mention)
        };

        bot.send_message(msg.chat.id, response)
            .parse_mode(teloxide::types::ParseMode::MarkdownV2)
            .await?;
    }
    Ok(())
}
