use crate::database;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Database,
    Parse,
}
