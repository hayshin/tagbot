mod db;
use db::Database;

use teloxide::{prelude::*, utils::command::BotCommands};
use std::sync::Arc;
use serde::{Deserialize, Serialize};

type ChatId = i64;
type UserId = u64;
type TagName = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: UserId,
    pub username: Option<String>,
    pub first_name: String,
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
    let db = Database::new("tagbot.db").expect("Failed to initialize database");
    let storage: BotStorage = Arc::new(db);

    Command::repl(bot, move |bot: Bot, msg: Message, cmd: Command| {
        let storage = Arc::clone(&storage);
        async move {
            answer(bot, msg, cmd, storage).await?;
            Ok(())
        }
    })
    .await;
}

async fn answer(
    bot: Bot,
    msg: Message,
    cmd: Command,
    storage: BotStorage,
) -> ResponseResult<()> {
    let chat_id = msg.chat.id.0;

    // Check if it's a private chat and register the user
    if msg.chat.is_private() {
        if let Some(user) = msg.from() {
            storage.register_private_user(user.id.0).expect("DB error");
        }
    }

    let user_info = match msg.from() {
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

    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?;
        }
        Command::Mute => {
            if storage.mute_user(chat_id, &user_info).expect("DB error") {
                bot.send_message(msg.chat.id, "You have been muted and won't be called in group mentions")
                    .await?;
            } else {
                bot.send_message(msg.chat.id, "You are already muted")
                    .await?;
            }
        }
        Command::Left(mut tag_name) => {
            if tag_name.is_empty() {
                tag_name = "all".to_string();
            }

            if storage.leave_tag(chat_id, &tag_name, user_info.id).expect("DB error") {
                bot.send_message(
                    msg.chat.id,
                    format!("You have been removed from tag '{}'", tag_name),
                )
                .await?;
            } else {
                bot.send_message(
                    msg.chat.id,
                    format!("You are not in tag '{}' or it doesn't exist", tag_name),
                )
                .await?;
            }
        }
        Command::Join(mut tag_name) => {
            if tag_name.is_empty() {
                tag_name = "all".to_string();
            }

            if storage.join_tag(chat_id, &tag_name, &user_info).expect("DB error") {
                bot.send_message(
                    msg.chat.id,
                    format!("You have been added to tag '{}'", tag_name),
                )
                .await?;
            } else {
                bot.send_message(
                    msg.chat.id,
                    format!("You are already in tag '{}'", tag_name),
                )
                .await?;
            }
        }
        Command::List => {
            let tags = storage.list_tags(chat_id).expect("DB error");
            let muted_count = storage.get_muted_count(chat_id).expect("DB error");

            if tags.is_empty() {
                bot.send_message(msg.chat.id, format!("No tags exist in this group yet.\nMuted users: {}", muted_count)).await?;
            } else {
                let mut tag_list = Vec::new();
                for (tag_name, count) in tags {
                    tag_list.push(format!("• {} ({} users)", tag_name, count));
                }

                let message = format!("Tags in this group:\n{}\n\nMuted users: {}", tag_list.join("\n"), muted_count);
                bot.send_message(msg.chat.id, message).await?;
            }
        }
        Command::Call(tag_name) => {
            let is_all = tag_name.is_empty() || tag_name == "all";
            let users_to_call = if is_all {
                storage.get_all_non_muted_users(chat_id).expect("DB error")
            } else {
                storage.get_tag_users(chat_id, &tag_name).expect("DB error")
            };

            if users_to_call.is_empty() {
                let message = if is_all {
                    "No users to call in this group".to_string()
                } else {
                    format!("No users in tag '{}'", tag_name)
                };
                bot.send_message(msg.chat.id, message).await?;
            } else {
                let mentions: Vec<String> = users_to_call
                    .iter()
                    .map(|user| {
                        if let Some(username) = &user.username {
                            format!("@{}", username)
                        } else {
                            user.first_name.clone()
                        }
                    })
                    .collect();

                let message = if is_all {
                    format!("Calling all: {}", mentions.join(" "))
                } else {
                    format!("Calling tag '{}': {}", tag_name, mentions.join(" "))
                };

                bot.send_message(msg.chat.id, message).await?;

                // Also send direct messages to users who have started the bot privately
                let from_name = match msg.from() {
                    Some(user) => format!("{} ({})", user.first_name, msg.chat.title().unwrap_or("this group")),
                    None => "Someone in the group".to_string(),
                };

                for user in users_to_call {
                    if storage.is_private_user(user.id).expect("DB error") {
                        let dm_message = if is_all {
                            format!("🔔 You were called in {} as part of 'all' tag!", from_name)
                        } else {
                            format!("🔔 You were called in {} for tag '{}'!", from_name, tag_name)
                        };
                        // We use user.id as chat_id for private messages
                        let _ = bot.send_message(ChatId(user.id as i64), dm_message).await;
                    }
                }
            }
        }
    }

    Ok(())
}
    }

    Ok(())
}
