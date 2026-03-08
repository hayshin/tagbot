use crate::db::Database;
use crate::models::UserInfo;
use std::sync::Arc;
use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;

pub mod ask;
pub mod call;
pub mod join;
pub mod leave;
pub mod list;
pub mod mute;
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

    // 1. Explicit command/special keyword overrides
    let mapped = match word.as_str() {
        "аск" => "ask",
        "калл" | "колл" => "call",
        "лив" => "leave",
        "джоин" | "жоин" => "join",
        "лист" => "list",
        "мьют" | "мут" => "mute",
        "анмьют" | "анмут" => "unmute",
        "хелп" | "помощь" => "help",
        "алл" | "все" | "всё" => "all",
        _ => "",
    };

    if !mapped.is_empty() {
        return mapped.to_string();
    }

    // 2. Fallback to general transliteration for all other tags
    transliterate(&word)
}

fn transliterate(s: &str) -> String {
    let mut res = String::with_capacity(s.len());
    for c in s.chars() {
        let replacement = match c {
            'а' => "a",
            'б' => "b",
            'в' => "v",
            'г' => "g",
            'д' => "d",
            'е' => "e",
            'ё' => "e",
            'ж' => "zh",
            'з' => "z",
            'и' => "i",
            'й' => "y",
            'к' => "k",
            'л' => "l",
            'м' => "m",
            'н' => "n",
            'о' => "o",
            'п' => "p",
            'р' => "r",
            'с' => "s",
            'т' => "t",
            'у' => "u",
            'ф' => "f",
            'х' => "h",
            'ц' => "c",
            'ч' => "ch",
            'ш' => "sh",
            'щ' => "sch",
            'ъ' => "",
            'ы' => "y",
            'ь' => "",
            'э' => "e",
            'ю' => "yu",
            'я' => "ya",
            _ => {
                res.push(c);
                continue;
            }
        };
        res.push_str(replacement);
    }
    res
}

impl Tag {
    pub fn new(raw: String) -> Self {
        let normalized = normalize_word(&raw);
        if normalized.is_empty() {
            Self("all".to_string())
        } else {
            Self(normalized)
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
    #[command(description = "замутить тег: /mute <название_тега>")]
    Mute(String),
    #[command(description = "размутить тег: /unmute <название_тега>")]
    Unmute(String),
    #[command(description = "выйти из тега: /leave [название_тега] (по умолчанию 'all')")]
    Leave(String),
    #[command(description = "войти в тег: /join [название_тега] (по умолчанию 'all')")]
    Join(String),
    #[command(description = "вызвать всех участников тега: /call [название_тега] (по умолчанию 'all')")]
    Call(String),
    #[command(description = "выбрать случайного участника тега: /ask [название_тега] (по умолчанию 'all')")]
    Ask(String),
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
        Command::Mute(arg) => {
            mute::handle_mute(ctx, arg).await?;
        }
        Command::Unmute(arg) => {
            mute::handle_unmute(ctx, arg).await?;
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
        Command::Ask(input) => {
            ask::handle_ask(ctx, input).await?;
        }
    }
    Ok(())
}
