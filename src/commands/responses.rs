use async_trait::async_trait;
use teloxide::prelude::*;
use teloxide::types::{ParseMode, Recipient};
use teloxide::utils::markdown;

#[async_trait]
pub trait BotResponseExt {
    async fn send_error_msg<C>(&self, chat_id: C, message: &str) -> anyhow::Result<()>
    where
        C: Into<Recipient> + Send;

    async fn send_success_msg<C>(&self, chat_id: C, message: &str) -> anyhow::Result<()>
    where
        C: Into<Recipient> + Send;
}

#[async_trait]
impl BotResponseExt for Bot {
    async fn send_error_msg<C>(&self, chat_id: C, message: &str) -> anyhow::Result<()>
    where
        C: Into<Recipient> + Send,
    {
        self.send_message(chat_id, format!("❌ {}", markdown::escape(message)))
            .parse_mode(ParseMode::MarkdownV2)
            .await?;
        Ok(())
    }

    async fn send_success_msg<C>(&self, chat_id: C, message: &str) -> anyhow::Result<()>
    where
        C: Into<Recipient> + Send,
    {
        self.send_message(chat_id, format!("✅ {}", markdown::escape(message)))
            .parse_mode(ParseMode::MarkdownV2)
            .await?;
        Ok(())
    }
}
