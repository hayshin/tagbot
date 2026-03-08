use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;
use std::sync::Arc;
use crate::db::Database;
use crate::models::UserInfo;

pub mod call;
pub mod join;
pub mod leave;
pub mod list;
pub mod mute;
pub mod ask;

pub struct CommandContext {
    pub bot: Bot,
    pub msg: Message,
    pub db: Arc<Database>,
    pub user: UserInfo,
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Available commands:")]
pub enum Command {
    #[command(description = "mute yourself: /mute [ask] (default to 'all')")]
    Mute(String),
    #[command(description = "unmute yourself: /unmute [ask] (default to 'all')")]
    Unmute(String),
    #[command(description = "leave a tag: /left [tag_name] (defaults to 'all')")]
    Left(String),
    #[command(description = "join a tag: /join [tag_name] (defaults to 'all')")]
    Join(String),
    #[command(description = "call all users in a tag: /call [tag_name] (defaults to 'all')")]
    Call(String),
    #[command(description = "ask a question to a tag: /ask [tag_name] [question] (defaults to 'all')")]
    Ask(String),
    #[command(description = "list all tags in this group")]
    List,
    #[command(description = "show available commands")]
    Help,
}

pub async fn handle_command(ctx: CommandContext, cmd: Command) -> anyhow::Result<()> {
    match cmd {
        Command::Help => {
            ctx.bot.send_message(ctx.msg.chat.id, Command::descriptions().to_string()).await?;
        }
        Command::Mute(arg) => {
            mute::handle_mute(ctx, arg).await?;
        }
        Command::Unmute(arg) => {
            mute::handle_unmute(ctx, arg).await?;
        }
        Command::Join(tag) => {
            join::handle_join(ctx, tag).await?;
        }
        Command::Left(tag) => {
            leave::handle_leave(ctx, tag).await?;
        }
        Command::List => {
            list::handle_list(ctx).await?;
        }
        Command::Call(tag) => {
            call::handle_call(ctx, tag).await?;
        }
        Command::Ask(input) => {
            ask::handle_ask(ctx, input).await?;
        }
    }
    Ok(())
}

pub fn normalize_tag(tag: String) -> String {
    let t = tag.trim();
    if t.is_empty() {
        "all".to_string()
    } else {
        t.to_lowercase()
    }
}
