use crate::commands::{CommandContext, Tag};
use futures::future::join_all;
use teloxide::prelude::*;
use teloxide::utils::markdown;

pub async fn handle_call(ctx: CommandContext, tag: Tag) -> anyhow::Result<()> {
    let users_to_call = ctx
        .db
        .get_tag_users(ctx.msg.chat.id.0, tag.as_ref().to_string())
        .await?;

    if users_to_call.is_empty() {
        ctx.bot
            .send_message(
                ctx.msg.chat.id,
                format!(
                    "No users in tag '{}' (or they are all muted)",
                    markdown::escape(tag.as_ref())
                ),
            )
            .await?;
    } else {
        let mentions: Vec<String> = users_to_call
            .iter()
            .map(|user| user.info.mention())
            .collect();

        let message = format!(
            "Calling tag '{}': {}",
            markdown::escape(tag.as_ref()),
            mentions.join(" ")
        );

        ctx.bot
            .send_message(ctx.msg.chat.id, message)
            .parse_mode(teloxide::types::ParseMode::MarkdownV2)
            .await?;

        // Also send direct messages to users who have started the bot privately
        let from_chat_title = ctx.msg.chat.title().unwrap_or("this group");
        let from_name = format!(
            "{} ({})",
            markdown::escape(
                &ctx.msg
                    .from
                    .as_ref()
                    .map(|u| u.first_name.clone())
                    .unwrap_or_else(|| "Someone".to_string())
            ),
            markdown::escape(from_chat_title)
        );

        let dm_futures = users_to_call
            .into_iter()
            .filter(|u| u.is_private)
            .map(|user| {
                let bot = ctx.bot.clone();
                let tag_name = tag.as_ref().to_string();
                let from_name = from_name.clone();
                async move {
                    let dm_message = format!(
                        "🔔 You were called in {} for tag '{}'!",
                        from_name, tag_name
                    );
                    let _ = bot
                        .send_message(teloxide::types::ChatId(user.info.id as i64), dm_message)
                        .await;
                }
            });

        join_all(dm_futures).await;
    }
    Ok(())
}
