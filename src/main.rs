mod commands;
mod db;
mod models;

use commands::Command;
use db::Database;
use models::UserInfo;

use std::sync::Arc;
use teloxide::prelude::*;

type BotStorage = Arc<Database>;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting tag bot...");

    let bot = Bot::from_env();
    let me = bot.get_me().await.expect("Failed to get bot info");

    let db_path = std::env::var("DATABASE_URL").unwrap_or_else(|_| "tagbot.db".to_string());
    let db = Database::new(&db_path)
        .await
        .expect("Failed to initialize database");
    let storage: BotStorage = Arc::new(db);

    let handler = Update::filter_message()
        .branch(dptree::entry().filter_command::<Command>().endpoint(answer))
        .branch(dptree::entry().endpoint(handle_maybe_plain_command));

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![storage, me])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn answer(bot: Bot, msg: Message, cmd: Command, storage: BotStorage) -> anyhow::Result<()> {
    process_command(bot, msg, cmd, storage).await
}

async fn handle_maybe_plain_command(
    bot: Bot,
    msg: Message,
    storage: BotStorage,
) -> anyhow::Result<()> {
    if let Some(text) = msg.text() {
        let (cmd_name_raw, args) = match text.split_once(' ') {
            Some((name, args)) => (name, args),
            None => (text, ""),
        };

        let cmd_name = commands::normalize_word(cmd_name_raw);
        let cmd = match cmd_name.as_str() {
            "ask" => Some(Command::Ask(args.to_string())),
            "call" => Some(Command::Call(args.to_string())),
            "join" => Some(Command::Join(args.to_string())),
            "leave" => Some(Command::Leave(args.to_string())),
            "list" => Some(Command::List),
            "mute" => Some(Command::Mute(args.to_string())),
            "unmute" => Some(Command::Unmute(args.to_string())),
            "help" => Some(Command::Help),
            _ => None,
        };

        if let Some(cmd) = cmd {
            return process_command(bot, msg, cmd, storage).await;
        }
    }
    Ok(())
}

async fn process_command(
    bot: Bot,
    msg: Message,
    cmd: Command,
    storage: BotStorage,
) -> anyhow::Result<()> {
    // Basic user registration and upsert
    if let Some(user) = &msg.from {
        let user_info = UserInfo {
            id: user.id.0,
            username: user.username.clone(),
            first_name: user.first_name.clone(),
        };

        if msg.chat.is_private() {
            storage.register_private_user(&user_info).await?;
        } else {
            storage.upsert_user(&user_info).await?;
        }

        let ctx = commands::CommandContext {
            bot,
            msg,
            db: storage,
            user: user_info,
        };

        commands::handle_command(ctx, cmd).await?;
    } else {
        bot.send_message(msg.chat.id, "Не удалось определить пользователя")
            .await?;
    }

    Ok(())
}
