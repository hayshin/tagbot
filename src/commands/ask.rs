use teloxide::prelude::*;
use teloxide::utils::markdown;
use rand::seq::SliceRandom;
use crate::commands::{Tag, CommandContext};

pub async fn handle_ask(ctx: CommandContext, input: String) -> anyhow::Result<()> {
    let input = input.trim();
    if input.is_empty() {
        return pick_and_respond(ctx, Tag::new("all".to_string()), "").await;
    }

    let mut parts = input.splitn(2, ' ');
    let first_word = parts.next().unwrap_or("");
    let rest = parts.next().unwrap_or("");

    let potential_tag = Tag::new(first_word.to_string());
    
    // Check if the first word is a known tag
    if ctx.db.tag_exists(ctx.msg.chat.id.0, potential_tag.as_ref().to_string()).await? {
        pick_and_respond(ctx, potential_tag, rest).await
    } else {
        // Not a tag, so the whole input is the question for the "all" tag
        pick_and_respond(ctx, Tag::new("all".to_string()), input).await
    }
}

async fn pick_and_respond(ctx: CommandContext, tag: Tag, question: &str) -> anyhow::Result<()> {
    let users = ctx.db.get_tag_users(ctx.msg.chat.id.0, tag.as_ref().to_string(), Some("ask".to_string())).await?;

    if users.is_empty() {
        ctx.bot.send_message(ctx.msg.chat.id, format!("No users in tag '{}' (or they are all muted for /ask)", markdown::escape(tag.as_ref()))).await?;
    } else {
        let picked_user = users.choose(&mut rand::thread_rng()).unwrap();
        let mention = picked_user.info.mention();

        let response = if question.is_empty() {
            format!("The chosen one from tag '{}' is {}!", markdown::escape(tag.as_ref()), mention)
        } else {
            format!(r"{} \- it's {}!", markdown::escape(question), mention)
        };

        ctx.bot.send_message(ctx.msg.chat.id, response)
            .parse_mode(teloxide::types::ParseMode::MarkdownV2)
            .await?;
    }
    Ok(())
}
