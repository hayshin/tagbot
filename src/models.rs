use serde::{Deserialize, Serialize};
use teloxide::utils::markdown;

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
            Some(username) => format!("@{}", markdown::escape(username)),
            None => {
                format!(
                    "[{}](tg://user?id={})",
                    markdown::escape(&self.first_name),
                    self.id
                )
            }
        }
    }
}
