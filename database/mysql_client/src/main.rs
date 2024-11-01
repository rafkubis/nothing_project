extern crate paho_mqtt as mqtt;
use crate::client::Client;
use env_logger::{Builder, Target};
use futures::{executor::block_on, stream::StreamExt};
use mqtt::QOS_0;
use mysql::serde_json;
use std::env;
use std::fs::File;
use std::time::Duration;

use crate::database::*;
pub mod client;
pub mod database;
pub mod rest;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let name = env!("CARGO_PKG_NAME");
        let log_filepath = args[1].to_owned() + "/" + name + ".log";
        let file: Box<File> = Box::new(File::create(log_filepath).unwrap());
        Builder::from_default_env()
            .target(Target::Pipe(file))
            .init();
    } else {
        Builder::from_default_env()
            .target(Target::Stdout)
            .filter_level(log::LevelFilter::Debug)
            .init();
    }
    log::info!("Starting application");

    let mut conn = database::open_sql_connection();
    create_table_if_not_exist(&mut conn);
    if let Err(err) = block_on(async {
        let handle = &mut |msg| {
            handle_message(msg, &mut conn);
        };
        let mut mqtt_client = client::MqttClient::open(handle).await;
        mqtt_client.receive().await;

        Ok::<(), mqtt::Error>(())
    }) {
        log::error!("{err}");
    };

    println!("Hello, world!");
}

fn handle_message(msg: mqtt::Message, conn: &mut mysql::PooledConn) {
    let parsed = serde_json::from_str::<rest::Root>(msg.payload_str().as_ref());
    match parsed {
        Ok(parsed) => {
            log::info!("Parsed: {:?}", parsed);

            let temperature = calculate_temperature(parsed);
            let formatted_date_time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            drop_querry(conn, temperature, &formatted_date_time);
        }
        Err(..) => {
            log::error!("Error parsing message: {:?}", msg.payload_str());
        }
    }
}

fn calculate_temperature(parsed: rest::Root) -> f32 {
    let mut temperature = (parsed.multiSensor.sensors[0].value / 100) as f32;
    temperature += (parsed.multiSensor.sensors[0].value as f32 - temperature * 100.0) / 100.0;
    temperature
}
