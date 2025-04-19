//extern crate paho_mqtt as mqtt;
use std::env;
use tokio;
pub mod app;
pub mod client;
pub mod database;
pub mod logger;
pub mod message_handler;
//pub mod json_multisensor;

fn get_log_path() -> Option<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let name = env!("CARGO_PKG_NAME");
        return Some(args[1].to_owned() + "/" + name + ".log");
    }
    None
}

#[tokio::main]
async fn main() {
    logger::init_logger(get_log_path());
    app::app().await;
}
mod integration_test;
