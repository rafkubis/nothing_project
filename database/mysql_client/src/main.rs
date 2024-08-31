extern crate paho_mqtt as mqtt;
use chrono;
use env_logger::{Builder, Target};
use mysql::serde_json;
use std::env;
use std::fs::File;
use std::time::Duration;

use crate::database::*;
pub mod database;
pub mod rest;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let name = env!("CARGO_PKG_NAME");
        let log_filepath = args[1].to_owned() + "/" + name + ".log";
        let file = Box::new(File::create(log_filepath).unwrap());
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

    let cli = connect_and_publish_message();

    let sub: mqtt::ServerResponse = cli.subscribe("test/topic", 0).expect("R");
    let receiver = cli.start_consuming();
    receiver.iter().for_each(|msg| match msg {
        None => log::debug!("None"),
        Some(value) => {
            let parsed = serde_json::from_str::<rest::Root>(value.payload_str().as_ref());
            match parsed {
                Ok(parsed) => {
                    log::info!("Parsed: {:?}", parsed);

                    let temperature = calculate_temperature(parsed);
                    let formatted_date_time =
                        chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
                    drop_querry(&mut conn, temperature, &formatted_date_time);
                }
                Err(e) => {
                    log::error!("Error: {:?}", e);
                }
            }
        }
    });

    println!("Hello, world!");
}

fn connect_and_publish_message() -> mqtt::Client {
    let cli = mqtt::Client::new("tcp://mosquitto:1883").unwrap();

    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(20))
        .clean_session(true)
        .finalize();

    if let Err(e) = cli.connect(conn_opts) {
        log::debug!("Unable to connect:\n\t{:?}", e);
    }

    let msg = mqtt::Message::new("test", "Hello world!", 0);
    if let Err(e2) = cli.publish(msg) {
        log::debug!("Unable to publish message:\n\t{:?}", e2);
    }

    cli
}

fn calculate_temperature(parsed: rest::Root) -> f32 {
    let mut temperature = (parsed.multiSensor.sensors[0].value / 100) as f32;
    temperature += (parsed.multiSensor.sensors[0].value as f32 - temperature * 100.0) / 100.0;
    temperature
}
