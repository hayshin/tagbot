use serde::{Deserialize, Serialize};
use teloxide::utils::html;

pub type UserId = u64;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: UserId,
    pub username: Option<String>,
    pub first_name: String,
}

impl UserInfo {
    pub fn mention(&self) -> String {
        match &self.username {
            Some(username) => format!("@{}", html::escape(username)),
            None => {
                format!(
                    "<a href=\"tg://user?id={}\">{}</a>",
                    self.id,
                    html::escape(&self.first_name)
                )
            }
        }
    }
}
