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

    pub async fn list_task(&self) -> Result<(), RequestError> {
        Ok(())
    }

    pub async fn choose_task(&self, page_num: usize) -> Result<(), RequestError> {
        interface_tg::send_markup(
            "text",
            self.get_markup_for_tasks(page_num),
            self.bot,
            self.chat_id,
        )
        .await
    }

    pub fn get_markup_for_tasks(&self, num: usize) -> InlineKeyboardMarkup {
        let mut markup = InlineKeyboardMarkup::default();
        let mut last_page = false;
        let name = "hello";
        let data = vec!["one", "two", "three", "next", "end"];
        for chunk in data.chunks(num) {
            markup = markup.append_row(
                chunk
                    .iter()
                    .copied()
                    .map(|name| {
                        InlineKeyboardButton::new(
                            name,
                            InlineKeyboardButtonKind::CallbackData("set data".to_owned() + name),
                        )
                    })
                    .collect::<Vec<_>>(),
            );
        }
        let mut move_buttons = vec![];
        if num > 0 {
            move_buttons.push(InlineKeyboardButton::new(
                "⬅️",
                InlineKeyboardButtonKind::CallbackData(
                    "seltz::page::".to_owned() + &(num - 1).to_string(),
                ),
            ))
        }
        if !last_page {
            move_buttons.push(InlineKeyboardButton::new(
                "➡️",
                InlineKeyboardButtonKind::CallbackData(
                    "seltz::page::".to_owned() + &(num + 1).to_string(),
                ),
            ))
        }
        markup.append_row(move_buttons)
    }
}
