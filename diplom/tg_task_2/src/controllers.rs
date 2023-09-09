use crate::{interface_tg, test_tasks};

use teloxide::prelude::*;
use teloxide::types::MessageId;
use teloxide::types::{
    InlineKeyboardButton, InlineKeyboardButtonKind, InlineKeyboardMarkup, KeyboardButton,
    KeyboardMarkup,
};
use teloxide::RequestError;

#[derive(Debug)]
pub struct TgMessageController<'a> {
    pub bot: &'a Bot,
    pub msg_id: MessageId,
    pub chat_id: ChatId,
    pub user_id: UserId,
    pub event: EventMessage,
}

pub struct TgCallbackController<'a> {
    pub msg_ctl: TgMessageController<'a>,
    pub cb_id: &'a str,
}

#[derive(Debug)]
pub enum EventMessage {
    StartEvent,
    SelectTaskForDelete,
    SelectTaskShow,
}

impl TgMessageController<'_> {
    pub async fn reply<R: ToString>(&self, response: R) -> Result<(), RequestError> {
        interface_tg::send_silent_message(&response.to_string(), self.bot, self.chat_id).await
    }

    pub async fn show_task(&mut self, page_num: usize) -> Result<(), RequestError> {
        self.event = EventMessage::SelectTaskShow;
        interface_tg::send_markup(
            "SelectTaskShow",
            self.get_markup_for_tasks(page_num),
            self.bot,
            self.chat_id,
        )
        .await
    }

    pub async fn delete_task(&mut self, page_num: usize) -> Result<(), RequestError> {
        self.event = EventMessage::SelectTaskForDelete;
        interface_tg::send_markup(
            "SelectTaskForDelete",
            self.get_markup_for_tasks(page_num),
            self.bot,
            self.chat_id,
        )
        .await
    }

    pub async fn new_task(&mut self) -> Result<(), RequestError> {
        println!("{:#?} new_task", self.event);
        self.reply(String::from("InsertName Task")).await
    }

    pub fn get_markup_for_tasks(&self, num: usize) -> InlineKeyboardMarkup {
        let mut markup = InlineKeyboardMarkup::default();
        let mut last_page = false;
        if let Some(data) = test_tasks::get_tasks_data(num) {
            for chunk in data.chunks(2) {
                markup = markup.append_row(
                    chunk
                        .iter()
                        .copied()
                        .map(|current| {
                            let name = current.name;
                            let mut mark = match self.event {
                                EventMessage::SelectTaskForDelete => {
                                    "select::task::delete".to_owned()
                                }
                                EventMessage::SelectTaskShow => "select::task::showtask".to_owned(),
                                _ => panic!(),
                            };
                            mark = mark + &current.id.to_string();
                            InlineKeyboardButton::new(
                                name,
                                InlineKeyboardButtonKind::CallbackData(mark),
                            )
                        })
                        .collect::<Vec<_>>(),
                );
            }
        } else {
            last_page = true;
        }
        let mut move_buttons = vec![];
        if num > 0 {
            move_buttons.push(InlineKeyboardButton::new(
                "⬅️",
                InlineKeyboardButtonKind::CallbackData(
                    "select::page::".to_owned() + &(num - 1).to_string(),
                ),
            ))
        }
        if !last_page {
            move_buttons.push(InlineKeyboardButton::new(
                "➡️",
                InlineKeyboardButtonKind::CallbackData(
                    "select::page::".to_owned() + &(num + 1).to_string(),
                ),
            ))
        }
        markup.append_row(move_buttons)
    }

    pub async fn select_tasks_page(&self, page_num: usize) -> Result<(), RequestError> {
        interface_tg::edit_markup(
            self.get_markup_for_tasks(page_num),
            self.bot,
            self.msg_id,
            self.chat_id,
        )
        .await
    }

    pub async fn show_task_data(&self, task_id: i32) -> Result<(), RequestError> {
        if let Some(data) = test_tasks::get_task_data_by_id(task_id) {
            let mut out: String = String::from("NameTask: ");
            out += data.name;
            out += "\n";
            out += "Start Date: ";
            out += data.start;
            out += "\n";
            out += "End  Date: ";
            out += data.end;
            self.reply(out).await
        } else {
            Ok(())
        }
    }
}

impl TgCallbackController<'_> {}
