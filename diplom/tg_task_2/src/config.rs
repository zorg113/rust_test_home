use std::{ffi::OsString, path::PathBuf};

use clap::Parser;
use directories::BaseDirs;

lazy_static::lazy_static! {
    pub static ref CONF:Conf = parse_args();
}

#[derive(Parser)]
#[command(author,version,about, long_about = None)]
pub struct Conf {
    #[arg(
        short,
        long,
        env = "TASK_DB",
        value_name = "FILE",
        help = "Path to SQLITE database file",
        default_value = get_default_database_file()
    )]
    pub database: PathBuf,
    #[arg(short, long, value_name = "BOT TOKEN", env = "BOT_TOKEN")]
    pub token: String,
}

pub fn parse_args() -> Conf {
    Conf::parse()
}

fn get_default_database_file() -> OsString {
    let db_name = "task.db";
    match BaseDirs::new() {
        Some(base_dirs) => base_dirs.data_dir().join(db_name).into(),
        None => db_name.into(),
    }
}
