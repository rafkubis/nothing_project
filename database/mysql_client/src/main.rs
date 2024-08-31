extern crate paho_mqtt as mqtt;
use chrono;
use env_logger::{Builder, Target};
use log::info;
use mqtt::SuccessCallback;
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

    let cli = connect_and_publish_message();
    let token= cli.subscribe("test/topic", mqtt::QOS_1);
    cli.set_message_callback(|cli, msg| {
        log::info!("message callback");
    });
    while true{

    }



    println!("Hello, world!");
}

fn connect_and_publish_message() -> mqtt::AsyncClient {
    let cli = mqtt::AsyncClient::new("tcp://mosquitto:1883").unwrap();

    let lwt = mqtt::Message::new(
        "test/lwt",
        "[LWT] Async subscriber v5 lost connection",
        mqtt::QOS_1,
    );

    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(20))
        .clean_session(true)
        .will_message(lwt)
        .finalize();

    cli.connect_with_callbacks(
        conn_opts,
        |cli, result| {
            log::info!("Success callback {result}");
        },
        |cli, result, reslut2| {
            log::info!("Failure callback {result}, {reslut2}");
        },
    );



    /*    let msg = mqtt::Message::new("test", "Hello world!", 0);
        if let Err(e2) = cli.publish(msg) {
            log::debug!("Unable to publish message:\n\t{:?}", e2);
        }
    */
    cli
}

fn calculate_temperature(parsed: rest::Root) -> f32 {
    let mut temperature = (parsed.multiSensor.sensors[0].value / 100) as f32;
    temperature += (parsed.multiSensor.sensors[0].value as f32 - temperature * 100.0) / 100.0;
    temperature
}
