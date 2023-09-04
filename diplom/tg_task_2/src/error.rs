use crate::database;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Database(database::Error),
    Parse(String),
    CronParse,
    TeloxideRequest(teloxide::RequestError),
    UnmatchedQuery(teloxide::types::CallbackQuery),
    NoQueryData(teloxide::types::CallbackQuery),
    NoQueryMessage(teloxide::types::CallbackQuery),
    UserNotFound(teloxide::types::Message),
}

impl From<teloxide::RequestError> for Error {
    fn from(err: teloxide::RequestError) -> Self {
        Self::TeloxideRequest(err)
    }
}

impl From<database::Error> for Error {
    fn from(err: database::Error) -> Self {
        Self::Database(err)
    }
}
