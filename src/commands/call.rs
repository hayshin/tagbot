use crate::commands::{CommandContext, Tag};
use futures::future::join_all;
use teloxide::prelude::*;
use teloxide::utils::html;

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
                    "В теге '{}' нет пользователей",
                    html::escape(tag.as_ref())
                ),
            )
            .await?;
    } else {
        let mentions: Vec<String> = users_to_call
            .iter()
            .map(|user| user.info.mention())
            .collect();

        let message = format!(
            "Вызываем тег '{}': {}",
            html::escape(tag.as_ref()),
            mentions.join(" ")
        );

        ctx.bot
            .send_message(ctx.msg.chat.id, message)
            .parse_mode(teloxide::types::ParseMode::Html)
            .await?;

        // Also send direct messages to users who have started the bot privately
        let group_name = ctx.msg.chat.title().unwrap_or("этой группе");
        let caller_name = ctx.msg
            .from
            .as_ref()
            .map(|u| u.first_name.clone())
            .unwrap_or_else(|| "Кто-то".to_string());

        let dm_futures = users_to_call
            .into_iter()
            .filter(|u| u.is_private)
            .map(|user| {
                let bot = ctx.bot.clone();
                let tag_name = tag.as_ref().to_string();
                let caller_name = html::escape(&caller_name);
                let group_name = html::escape(group_name);
                async move {
                    let dm_message = format!(
                        "🔔 Вас вызвал/а {} в группе {} по тегу '{}'!",
                        caller_name, group_name, tag_name
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
