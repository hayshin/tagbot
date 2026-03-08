use teloxide::prelude::*;
use teloxide::utils::markdown;
use rand::seq::SliceRandom;
use crate::commands::{Tag, CommandContext, BotResponseExt};

pub async fn handle_ask(ctx: CommandContext, input: String) -> anyhow::Result<()> {
    let input = input.trim();
    
    let mut tag = Tag::new("all".to_string());
    let mut question = input.to_string();
    
    let users = if !input.is_empty() {
        let mut parts = input.splitn(2, ' ');
        let first_word = parts.next().unwrap_or("");
        let rest = parts.next().unwrap_or("");
        let potential_tag = Tag::new(first_word.to_string());
        
        let users = ctx.db.get_tag_users(ctx.msg.chat.id.0, potential_tag.to_string(), Some("ask".to_string())).await?;
        if !users.is_empty() {
            tag = potential_tag;
            question = rest.to_string();
            users
        } else {
            // Fallback to "all" tag with full input as question
            ctx.db.get_tag_users(ctx.msg.chat.id.0, tag.to_string(), Some("ask".to_string())).await?
        }
    } else {
        // Empty input: use "all" tag
        question = "".to_string();
        ctx.db.get_tag_users(ctx.msg.chat.id.0, tag.to_string(), Some("ask".to_string())).await?
    };

    if users.is_empty() {
        ctx.bot.send_error_msg(
            ctx.msg.chat.id, 
            &format!("No users in tag '{}' (or they are all muted for /ask)", markdown::escape(tag.as_ref()))
        ).await?;
    } else {
        let picked_user = users.choose(&mut rand::thread_rng()).unwrap();
        let mention = picked_user.info.mention();

        let response = if question.is_empty() {
            format!("The chosen one from tag '{}' is {}!", markdown::escape(tag.as_ref()), mention)
        } else {
            format!(r"{} \- it's {}!", markdown::escape(&question), mention)
        };

        ctx.bot.send_message(ctx.msg.chat.id, response)
            .parse_mode(teloxide::types::ParseMode::MarkdownV2)
            .await?;
    }
    Ok(())
}
