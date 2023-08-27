#[macro_use]
extern crate lazy_static;
extern crate pretty_env_logger;

mod error;
mod bot;
mod config;
mod database;
mod migration;

#[tokio::main]
async fn main() {
    bot::run().await;
}
