mod db;
use db::Database;

use teloxide::{prelude::*, utils::{command::BotCommands, markdown}, repls::CommandReplExt};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use futures::future::join_all;

type UserId = u64;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: UserId,
    pub username: Option<String>,
    pub first_name: String,
}

impl UserInfo {
    fn mention(&self) -> String {
        match &self.username {
            Some(username) => format!("@{}", markdown::escape(username)),
            None => {
                format!("[{}](tg://user?id={})", markdown::escape(&self.first_name), self.id)
            }
        }
    }
}

type BotStorage = Arc<Database>;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Available commands:")]
enum Command {
    #[command(description = "mute yourself - you won't be called unless explicitly tagged")]
    Mute,
    #[command(description = "leave a tag: /left [tag_name] (defaults to 'all')")]
    Left(String),
    #[command(description = "join a tag: /join [tag_name] (defaults to 'all')")]
    Join(String),
    #[command(description = "call all users in a tag: /call [tag_name] (defaults to 'all')")]
    Call(String),
    #[command(description = "list all tags in this group")]
    List,
    #[command(description = "show available commands")]
    Help,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting tag bot...");

    let bot = Bot::from_env();
    let db = Database::new("tagbot.db").await.expect("Failed to initialize database");
    let storage: BotStorage = Arc::new(db);

    Command::repl(bot, move |bot: Bot, msg: Message, cmd: Command| {
        let storage = Arc::clone(&storage);
        async move {
            if let Err(e) = answer(bot.clone(), msg.clone(), cmd, storage).await {
                log::error!("Error in answer: {:?}", e);
                let _ = bot.send_message(msg.chat.id, "An internal error occurred while processing your request.").await;
            }
            Ok(())
        }
    })
    .await;
}

fn normalize_tag(tag: String) -> String {
    let t = tag.trim();
    if t.is_empty() {
        "all".to_string()
    } else {
        t.to_lowercase()
    }
}

async fn answer(
    bot: Bot,
    msg: Message,
    cmd: Command,
    storage: BotStorage,
) -> anyhow::Result<()> {
    let chat_id = msg.chat.id.0;

    let user_info = match msg.from {
        Some(user) => UserInfo {
            id: user.id.0,
            username: user.username.clone(),
            first_name: user.first_name.clone(),
        },
        None => {
            bot.send_message(msg.chat.id, "Cannot identify user").await?;
            return Ok(());
        }
    };

    // Check if it's a private chat and register the user
    if msg.chat.is_private() {
        storage.register_private_user(&user_info).await?;
    }

    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?;
        }
        Command::Mute => {
            if storage.mute_user(chat_id, &user_info).await? {
                bot.send_message(msg.chat.id, "You have been muted and won't be called in group mentions")
                    .await?;
            } else {
                bot.send_message(msg.chat.id, "You are already muted")
                    .await?;
            }
        }
        Command::Left(tag_name) => {
            let tag_name = normalize_tag(tag_name);

            if storage.leave_tag(chat_id, tag_name.clone(), user_info.id).await? {
                bot.send_message(
                    msg.chat.id,
                    format!("You have been removed from tag '{}'", markdown::escape(&tag_name)),
                )
                .await?;
            } else {
                bot.send_message(
                    msg.chat.id,
                    format!("You are not in tag '{}' or it doesn't exist", markdown::escape(&tag_name)),
                )
                .await?;
            }
        }
        Command::Join(tag_name) => {
            let tag_name = normalize_tag(tag_name);

            if storage.join_tag(chat_id, tag_name.clone(), &user_info).await? {
                bot.send_message(
                    msg.chat.id,
                    format!("You have been added to tag '{}'", markdown::escape(&tag_name)),
                )
                .await?;
            } else {
                bot.send_message(
                    msg.chat.id,
                    format!("You are already in tag '{}'", markdown::escape(&tag_name)),
                )
                .await?;
            }
        }
        Command::List => {
            let tags = storage.list_tags(chat_id).await?;
            let muted_count = storage.get_muted_count(chat_id).await?;

            if tags.is_empty() {
                bot.send_message(msg.chat.id, format!("No tags exist in this group yet.\nMuted users: {}", muted_count)).await?;
            } else {
                let mut tag_list = Vec::new();
                for (tag_name, count) in tags {
                    tag_list.push(format!("• {} ({} users)", markdown::escape(&tag_name), count));
                }

                let message = format!("Tags in this group:\n{}\n\nMuted users: {}", tag_list.join("\n"), muted_count);
                bot.send_message(msg.chat.id, message).await?;
            }
        }
        Command::Call(tag_name) => {
            let tag_name = normalize_tag(tag_name);
            let users_to_call = storage.get_tag_users(chat_id, tag_name.clone()).await?;

            if users_to_call.is_empty() {
                bot.send_message(msg.chat.id, format!("No users in tag '{}'", markdown::escape(&tag_name))).await?;
            } else {
                let mentions: Vec<String> = users_to_call
                    .iter()
                    .map(|user| user.mention())
                    .collect();

                let message = format!("Calling tag '{}': {}", markdown::escape(&tag_name), mentions.join(" "));

                bot.send_message(msg.chat.id, message)
                    .parse_mode(teloxide::types::ParseMode::MarkdownV2)
                    .await?;

                // Also send direct messages to users who have started the bot privately
                let from_chat_title = msg.chat.title().unwrap_or("this group");
                let from_name = format!("{} ({})", markdown::escape(&user_info.first_name), markdown::escape(from_chat_title));

                let dm_futures = users_to_call.into_iter().map(|user| {
                    let bot = bot.clone();
                    let storage = Arc::clone(&storage);
                    let tag_name = tag_name.clone();
                    let from_name = from_name.clone();
                    async move {
                        if let Ok(true) = storage.is_private_user(user.id).await {
                            let dm_message = format!("🔔 You were called in {} for tag '{}'!", from_name, tag_name);
                            let _ = bot.send_message(teloxide::types::ChatId(user.id as i64), dm_message).await;
                        }
                    }
                });

                join_all(dm_futures).await;
            }
        }
    }

    Ok(())
}
