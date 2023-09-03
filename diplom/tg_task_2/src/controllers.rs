use crate::interface_tg;

use teloxide::prelude::*;
use teloxide::types::MessageId;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardButtonKind, InlineKeyboardMarkup};
use teloxide::RequestError;

pub struct TgMessageController<'a> {
    pub bot: &'a Bot,
    pub msg_id: MessageId,
    pub chat_id: ChatId,
    pub user_id: UserId,
}

impl TgMessageController<'_> {
    pub async fn reply<R: ToString>(&self, response: R) -> Result<(), RequestError> {
        interface_tg::send_silent_message(&response.to_string(), self.bot, self.chat_id).await
    }

    pub async fn show_task(&self) -> Result<(), RequestError> {
        self.reply(String::from("Hello")).await
    }
}
