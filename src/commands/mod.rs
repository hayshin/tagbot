use crate::db::Database;
use crate::models::UserInfo;
use std::sync::Arc;
use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;

pub mod call;
pub mod join;
pub mod leave;
pub mod list;
pub mod responses;

pub use responses::BotResponseExt;

pub struct CommandContext {
    pub bot: Bot,
    pub msg: Message,
    pub db: Arc<Database>,
    pub user: UserInfo,
}

pub struct Tag(String);

pub fn normalize_word(word: &str) -> String {
    let word = word.trim().to_lowercase();

    match word.as_str() {
        "калл" | "колл" => "call".to_string(),
        "лив" => "leave".to_string(),
        "джоин" | "жоин" => "join".to_string(),
        "лист" => "list".to_string(),
        "хелп" | "помощь" => "help".to_string(),
        other => other.to_string(),
    }
}

impl Tag {
    pub fn new(raw: String) -> Self {
        let trimmed = raw.trim().to_lowercase();
        match trimmed.as_str() {
            "" | "all" | "все" | "всё" => Self("алл".to_string()),
            other => Self(other.to_string()),
        }
    }
}

impl std::fmt::Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for Tag {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Доступные команды:")]
pub enum Command {
    #[command(description = "начать работу с ботом")]
    Start,
    #[command(description = "выйти из тега: /leave [название_тега] (по умолчанию 'алл')")]
    Leave(String),
    #[command(description = "войти в тег: /join [название_тега] (по умолчанию 'алл')")]
    Join(String),
    #[command(description = "вызвать всех участников тега: /call [название_тега] (по умолчанию 'алл')")]
    Call(String),
    #[command(description = "показать список всех тегов в группе")]
    List,
    #[command(description = "показать справку по командам")]
    Help,
}


pub async fn handle_command(ctx: CommandContext, cmd: Command) -> anyhow::Result<()> {
    match cmd {
        Command::Start => {
            let welcome = format!(
                "Добро пожаловать, {}! 👋\n\nЯ бот для тегов. Вы можете вступать в теги, и вас будут вызывать, когда вы понадобитесь.\n\n{}",
                ctx.user.first_name,
                Command::descriptions()
            );
            ctx.bot.send_message(ctx.msg.chat.id, welcome).await?;
        }
        Command::Help => {
            ctx.bot
                .send_message(ctx.msg.chat.id, Command::descriptions().to_string())
                .await?;
        }
        Command::Join(tag) => {
            join::handle_join(ctx, Tag::new(tag)).await?;
        }
        Command::Leave(tag) => {
            leave::handle_leave(ctx, Tag::new(tag)).await?;
        }
        Command::List => {
            list::handle_list(ctx).await?;
        }
        Command::Call(tag) => {
            call::handle_call(ctx, Tag::new(tag)).await?;
        }
    }
    Ok(())
}
