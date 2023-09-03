use crate::database;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Database,
    Parse(String),
    CronParse,
    TeloxideRequest(teloxide::RequestError),
    UnmatchedQuery(teloxide::types::CallbackQuery),
    NoQueryData(teloxide::types::CallbackQuery),
    NoQueryMessage(teloxide::types::CallbackQuery),
    UserNotFound(teloxide::types::Message),
}
