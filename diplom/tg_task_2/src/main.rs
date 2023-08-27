#[macro_use]
extern crate lazy_static;
extern crate pretty_env_logger;

mod Error;
mod bot;
mod config;
mod database;

#[tokio::main]
async fn main() {
    bot::run().await;
}
