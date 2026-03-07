use teloxide::{prelude::*, utils::command::BotCommands};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};

type ChatId = i64;
type UserId = u64;
type TagName = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UserInfo {
    id: UserId,
    username: Option<String>,
    first_name: String,
}

type GroupData = HashMap<TagName, Vec<UserInfo>>;
type BotStorage = Arc<RwLock<HashMap<ChatId, GroupData>>>;

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
    let storage: BotStorage = Arc::new(RwLock::new(HashMap::new()));

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
            let mut storage = storage.write().await;
            let group = storage.entry(chat_id).or_insert_with(HashMap::new);
            let muted = group.entry("muted".to_string()).or_insert_with(Vec::new);

            if !muted.iter().any(|u| u.id == user_info.id) {
                muted.push(user_info.clone());
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

            let mut storage = storage.write().await;
            if let Some(group) = storage.get_mut(&chat_id) {
                if let Some(users) = group.get_mut(&tag_name) {
                    if let Some(pos) = users.iter().position(|u| u.id == user_info.id) {
                        users.remove(pos);
                        if users.is_empty() {
                            group.remove(&tag_name);
                        }
                        bot.send_message(
                            msg.chat.id,
                            format!("You have been removed from tag '{}'", tag_name),
                        )
                        .await?;
                    } else {
                        bot.send_message(
                            msg.chat.id,
                            format!("You are not in tag '{}'", tag_name),
                        )
                        .await?;
                    }
                } else {
                    bot.send_message(msg.chat.id, format!("Tag '{}' does not exist", tag_name))
                        .await?;
                }
            } else {
                bot.send_message(msg.chat.id, "No tags exist in this group yet")
                    .await?;
            }
        }
        Command::Join(mut tag_name) => {
            if tag_name.is_empty() {
                tag_name = "all".to_string();
            }

            let mut storage = storage.write().await;
            let group = storage.entry(chat_id).or_insert_with(HashMap::new);
            let users = group.entry(tag_name.clone()).or_insert_with(Vec::new);

            if !users.iter().any(|u| u.id == user_info.id) {
                users.push(user_info);
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
            let storage = storage.read().await;

            if let Some(group) = storage.get(&chat_id) {
                if group.is_empty() {
                    bot.send_message(msg.chat.id, "No tags exist in this group yet").await?;
                } else {
                    let mut tag_list = Vec::new();
                    for (tag_name, users) in group.iter() {
                        if tag_name != "muted" {
                            tag_list.push(format!("• {} ({} users)", tag_name, users.len()));
                        }
                    }

                    let muted_count = group.get("muted").map(|u| u.len()).unwrap_or(0);

                    let message = if tag_list.is_empty() {
                        format!("No custom tags yet.\nMuted users: {}", muted_count)
                    } else {
                        format!("Tags in this group:\n{}\n\nMuted users: {}", tag_list.join("\n"), muted_count)
                    };

                    bot.send_message(msg.chat.id, message).await?;
                }
            } else {
                bot.send_message(msg.chat.id, "No tags exist in this group yet").await?;
            }
        }
        Command::Call(tag_name) => {
            let storage = storage.read().await;

            if let Some(group) = storage.get(&chat_id) {
                let is_all = tag_name.is_empty() || tag_name == "all";
                let users_to_call: Vec<UserInfo> = if is_all {
                    // Call all users except muted
                    let muted_ids: Vec<UserId> = group
                        .get("muted")
                        .map(|users| users.iter().map(|u| u.id).collect())
                        .unwrap_or_default();

                    let mut unique_users = HashMap::new();
                    for users in group.values() {
                        for user in users {
                            if !muted_ids.contains(&user.id) {
                                unique_users.insert(user.id, user.clone());
                            }
                        }
                    }
                    unique_users.into_values().collect()
                } else {
                    // Call specific tag
                    group.get(&tag_name).cloned().unwrap_or_default()
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
                                // Fallback to first name if no username
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
                }
            } else {
                bot.send_message(msg.chat.id, "No tags exist in this group yet")
                    .await?;
            }
        }
    }

    Ok(())
}
