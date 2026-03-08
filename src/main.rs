mod db;
mod models;
mod commands;

use db::Database;
use models::UserInfo;
use commands::Command;

use teloxide::prelude::*;
use std::sync::Arc;

type BotStorage = Arc<Database>;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting tag bot...");

    let bot = Bot::from_env();
    let db_path = std::env::var("DATABASE_URL").unwrap_or_else(|_| "tagbot.db".to_string());
    let db = Database::new(&db_path).await.expect("Failed to initialize database");
    let storage: BotStorage = Arc::new(db);

    let handler = Update::filter_message()
        .branch(
            dptree::entry()
                .filter_command::<Command>()
                .endpoint(answer)
        );

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![storage])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn answer(
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
        bot.send_message(msg.chat.id, "Cannot identify user").await?;
    }

    Ok(())
}
