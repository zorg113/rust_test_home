use dotenv::dotenv;
use env_logger;
use futures_util::future::BoxFuture;
use serde::{Deserialize, Serialize};
use std::env;
use tgbot::{
    longpoll::LongPoll,
    methods::SendMessage,
    types::{InlineKeyboardButton, KeyboardButton, Message, Update, UpdateKind},
    Api, UpdateHandler,
};

struct Handler {
    api: Api,
}

#[derive(Deserialize, Serialize)]
struct ShowTasks {
    tasks: String,
}

impl ShowTasks {
    fn new<S: Into<String>>(value: S) -> Self {
        Self {
            tasks: value.into(),
        }
    }
}

#[derive(Deserialize, Serialize)]
struct AddTasks {
    tasks: String,
}

impl AddTasks {
    fn new<S: Into<String>>(value: S) -> Self {
        Self {
            tasks: value.into(),
        }
    }
}

/*
#[derive(Deserialize, Serialize)]
struct DeleteTasks {
    tasks: String,
}

impl DeleteTasks {
    fn new<S: Into<String>>(value: S) -> Self {
        Self {
            tasks: value.into(),
        }
    }
}*/

async fn handle_update(api: &Api, update: Update) -> Option<Message> {
    match update.kind {
        UpdateKind::Message(message) => {
            let chat_id = message.get_chat_id();
            if let Some(commands) = message.get_text().and_then(|text| text.get_bot_commands()) {
                let command = &commands[0];
                if command.command == "/start" {
                    let callback_show_tasks = ShowTasks::new("&data.clone()");
                    let callback_data = AddTasks::new("Введите что-то тама где-то почему-то как-то");
                    let method =
                        SendMessage::new(chat_id, "Менеджер задач").reply_markup(vec![vec![
                            // You also can use with_callback_data in order to pass a plain string
                            InlineKeyboardButton::with_callback_data_struct(
                                "Текущие задачи",
                                &callback_show_tasks,
                            )
                            .unwrap(),
                            InlineKeyboardButton::with_callback_data_struct(
                                "Добавить задачу",
                                &callback_data,
                            )
                            .unwrap(),
                            InlineKeyboardButton::with_callback_data_struct(
                                "Завершить задачу",
                                &callback_show_tasks,
                            )
                            .unwrap(),
                        ]]);
                    return Some(api.execute(method).await.unwrap());
                }
            }
        }
        UpdateKind::CallbackQuery(query) => {
            if let Some(ref message) = query.message {
                let chat_id = message.get_chat_id();
                // or query.data if you have passed a plain string
                let data = query.parse_data::<ShowTasks>().unwrap().unwrap();
                let method = SendMessage::new(chat_id, data.tasks);
                return Some(api.execute(method).await.unwrap());
            }
        }
        _ => {}
    }
    None
}

impl UpdateHandler for Handler {
    type Future = BoxFuture<'static, ()>;

    fn handle(&self, update: Update) -> Self::Future {
        let api = self.api.clone();
        Box::pin(async move {
            log::info!("Got an update: {:?}", update);
            if let Some(msg) = handle_update(&api, update).await {
                log::info!("Message sent: {:?}", msg);
            }
        })
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();
    let token = String::from("TOKEN");
    //let token = env::var("TGBOT_TOKEN").expect("TGBOT_TOKEN is not set");
    let api = Api::new(token).expect("Failed to create API");
    LongPoll::new(api.clone(), Handler { api }).run().await;
}
