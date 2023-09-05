use crate::config::CONF;
use crate::controllers::{TgCallbackController, TgMessageController};
use crate::database::Database;
use crate::error::Error;
use async_once::AsyncOnce;
use teloxide::payloads::AnswerPreCheckoutQuerySetters;
//use async_std::task;
//use lazy_static::lazy_static;
use teloxide::{prelude::*, types::MessageId, utils::command::BotCommands};

#[derive(BotCommands, Clone)]
#[command(description = "MainMenu", rename_rule = "lowercase")]
pub enum MainMenu {
    #[command(description = "AddTask")]
    NewTask,
    #[command(description = "ShowTask")]
    ShowTasks,
    #[command(description = "EndTask")]
    DeleteTask,
}

#[derive(BotCommands, Clone)]
#[command(description = "AddTaskMenu", rename_rule = "lowercase")]
pub enum AddTaskMenu {
    #[command(description = "AddNameTask")]
    AddNameTask,
    #[command(description = "Back")]
    MainMenu,
}

async fn pool_tasks(db: &Database, bot: Bot) {}

lazy_static! {
    static ref DATABASE: AsyncOnce<Database> = AsyncOnce::new(async {
        Database::new(&CONF.database).await.unwrap_or_else(|err| {
            panic!("Failed to connect to database {:?}: {}", CONF.database, err)
        })
    });
}

pub async fn run() {
    pretty_env_logger::init();
    log::info!("Starting task-bot!");
    DATABASE
        .get()
        .await
        .apply_migrations()
        .await
        .expect("Failed to apply migrations");
    let bot = Bot::new(&CONF.token);
    bot.set_my_commands(MainMenu::bot_commands())
        .await
        .expect("Failed to set bot commands");

    tokio::spawn(pool_tasks(DATABASE.get().await, bot.clone()));

    let handler = dptree::entry()
        .branch(
            Update::filter_message()
                .filter_command::<MainMenu>()
                .endpoint(command_handler),
        )
        .branch(Update::filter_message().endpoint(message_handler))
        .branch(Update::filter_callback_query().endpoint(callback_handler));

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn callback_handler(cb_query: CallbackQuery, bot: Bot) -> Result<(), Error> {
    println!("callback_handler");
    if let Some(cb_data) = &cb_query.data {
        let ctrl = TgCallbackController::new(&bot, &cb_query).await?;
        let msg_ctrl = &ctrl.msg_ctl;
        println!("input {}",cb_data);
        if let Some(page_num) = cb_data
            .strip_prefix("select::page::")
            .and_then(|x| x.parse::<usize>().ok())
        {
            msg_ctrl
                .select_tasks_page(page_num)
                .await
                .map_err(From::from)
        }
        else if let Some(task_id) = cb_data
             .strip_prefix("select::task::")
            .and_then(|x| x.parse::<i32>().ok())
        {
            println!("Select {}",task_id);
            Ok(())
        }
        else {
            println!("Ups");
            Ok(())
        }
    } else {
        Ok(())
    }
}

async fn command_handler(msg: Message, bot: Bot, cmd: MainMenu) -> Result<(), Error> {
    println!("Command handler");
    log::info!("Command handler");
    let cntrl = TgMessageController::from_msg(&bot, &msg).await?;
    match cmd {
        MainMenu::DeleteTask => cntrl.delete_task().await,
        MainMenu::ShowTasks => cntrl.show_task().await,
        MainMenu::NewTask => cntrl.new_task().await,
    }
    .map_err(From::from)
}

async fn message_handler(msg: Message, bot: Bot) -> Result<(), Error> {
    // check global state mashine 
    println!("message handler");
    log::info!("message handler");
    Ok(())
}

impl<'a> TgMessageController<'a> {
    pub async fn new(
        bot: &'a Bot,
        chat_id: ChatId,
        user_id: UserId,
        msg_id: MessageId,
    ) -> Result<TgMessageController<'a>, Error> {
        Ok(Self {
            //db: DATABASE.get().await,
            bot,
            chat_id,
            user_id,
            msg_id,
        })
    }

    pub async fn from_msg(bot: &'a Bot, msg: &Message) -> Result<TgMessageController<'a>, Error> {
        Self::new(
            bot,
            msg.chat.id,
            msg.from()
               .ok_or_else(|| Error::UserNotFound(msg.clone()))?
               .id,
            msg.id,
        )
        .await
    }

    pub async fn from_callback_query(
        bot: &'a Bot,
        cb_query: &CallbackQuery,
    ) -> Result<TgMessageController<'a>, Error> {
        let msg = cb_query
            .message
            .as_ref()
            .ok_or_else(|| Error::NoQueryMessage(cb_query.clone()))?;
        Self::new(bot, msg.chat.id, cb_query.from.id, msg.id).await
    }
}

impl<'a> TgCallbackController<'a> {
    pub async fn new(
        bot: &'a Bot,
        cb_query: &'a CallbackQuery,
    ) -> Result<TgCallbackController<'a>, Error> {
        Ok(Self {
            msg_ctl: TgMessageController::from_callback_query(bot, cb_query).await?,
            cb_id: &cb_query.id,
        })
    }
}
