use crate::commands::{BotResponseExt, CommandContext, Tag};
use rand::seq::SliceRandom;
use teloxide::prelude::*;
use teloxide::types::ReplyParameters;

pub async fn handle_ask(ctx: CommandContext, input: String) -> anyhow::Result<()> {
    let input = input.trim();

    let mut tag = Tag::new("all".to_string());

    let users = if !input.is_empty() {
        let first_word = input.split(' ').next().unwrap_or("");
        let potential_tag = Tag::new(first_word.to_string());

        let users = ctx
            .db
            .get_tag_users(
                ctx.msg.chat.id.0,
                potential_tag.to_string(),
            )
            .await?;
        if !users.is_empty() {
            tag = potential_tag;
            users
        } else {
            // Fallback to "all" tag
            ctx.db
                .get_tag_users(ctx.msg.chat.id.0, tag.to_string())
                .await?
        }
    } else {
        // Empty input: use "all" tag
        ctx.db
            .get_tag_users(ctx.msg.chat.id.0, tag.to_string())
            .await?
    };

    if users.is_empty() {
        ctx.bot
            .send_error_msg(
                ctx.msg.chat.id,
                &format!(
                    "No users in tag '{}'",
                    tag.as_ref()
                ),
            )
            .await?;
    } else {
        let picked_user = users.choose(&mut rand::thread_rng()).unwrap();
        let mention = picked_user.info.mention();

        let response = format!("It's {}!", mention);

        ctx.bot
            .send_message(ctx.msg.chat.id, response)
            .reply_parameters(ReplyParameters::new(ctx.msg.id))
            .parse_mode(teloxide::types::ParseMode::MarkdownV2)
            .await?;
    }
    Ok(())
}
