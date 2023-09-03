#[macro_use]
extern crate lazy_static;
extern crate pretty_env_logger;

mod bot;
mod config;
mod controllers;
mod database;
mod error;
mod interface_tg;
mod migration;
mod statemashine;

#[tokio::main]
async fn main() {
    bot::run().await;
}
