use teloxide::utils::command::BotCommands;

pub mod call;
pub mod join;
pub mod leave;
pub mod list;
pub mod mute;
pub mod unmute;
pub mod ask;

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

pub fn normalize_tag(tag: String) -> String {
    let t = tag.trim();
    if t.is_empty() {
        "all".to_string()
    } else {
        t.to_lowercase()
    }
}
