use teloxide::prelude::*;
use teloxide::utils::markdown;
use rand::seq::SliceRandom;
use crate::commands::{normalize_tag, CommandContext};

pub async fn handle_ask(ctx: CommandContext, input: String) -> anyhow::Result<()> {
    let input = input.trim();
    if input.is_empty() {
        return pick_and_respond(ctx, "all".to_string(), "").await;
    }

    let mut parts = input.splitn(2, ' ');
    let first_word = parts.next().unwrap_or("");
    let rest = parts.next().unwrap_or("");

    let normalized_first = normalize_tag(first_word.to_string());
    
    // Check if the first word is a known tag
    if ctx.db.tag_exists(ctx.msg.chat.id.0, normalized_first.clone()).await? {
        pick_and_respond(ctx, normalized_first, rest).await
    } else {
        // Not a tag, so the whole input is the question for the "all" tag
        pick_and_respond(ctx, "all".to_string(), input).await
    }
}

async fn pick_and_respond(ctx: CommandContext, tag_name: String, question: &str) -> anyhow::Result<()> {
    let users = ctx.db.get_tag_users(ctx.msg.chat.id.0, tag_name.clone(), Some("ask".to_string())).await?;

    if users.is_empty() {
        ctx.bot.send_message(ctx.msg.chat.id, format!("No users in tag '{}' (or they are all muted for /ask)", markdown::escape(&tag_name))).await?;
    } else {
        let picked_user = users.choose(&mut rand::thread_rng()).unwrap();
        let mention = picked_user.info.mention();

        let response = if question.is_empty() {
            format!("The chosen one from tag '{}' is {}!", markdown::escape(&tag_name), mention)
        } else {
            format!(r"{} \- it's {}!", markdown::escape(question), mention)
        };

        ctx.bot.send_message(ctx.msg.chat.id, response)
            .parse_mode(teloxide::types::ParseMode::MarkdownV2)
            .await?;
    }
    Ok(())
}
