use crate::config::CONF;
use crate::database::Database;
use crate::error::Error;
use async_once::AsyncOnce;
//use async_std::task;
//use lazy_static::lazy_static;
use teloxide::{
    prelude::*,
    types::MessageId,
    utils::command::{self, BotCommands},
};

#[derive(BotCommands, Clone)]
#[command(description = "Commands", rename_rule = "lowercase")]
pub enum Command {
    #[command(description = "create new task")]
    NewTask,
    #[command(description = "create new subtask")]
    NewSubTask,
    #[command(description = "show tasks")]
    ShowTask,
    #[command(description = "delete task")]
    DeleteTask,
    #[command(description = "rename task")]
    RenameTask,
    #[command(description = "change task")]
    ChangeTask,
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
    bot.set_my_commands(Command::bot_commands())
        .await
        .expect("Failed to set bot commands");

    tokio::spawn(pool_tasks(DATABASE.get().await, bot.clone()));

    let handler = dptree::entry()
        .branch(
            Update::filter_message()
                .filter_command::<Command>()
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
    Err(Error::Database)
}

async fn command_handler(msg: Message, bot: Bot, cmd: Command) -> Result<(), Error> {
    Err(Error::Database)
}

async fn message_handler(msg: Message, bot: Bot) -> Result<(), Error> {
    Err(Error::Database)
}
