extern crate paho_mqtt as mqtt;
use crate::client::Client;
use env_logger::fmt::Formatter;
use env_logger::{Builder, Target};
use log::Record;
use testcontainers::core::Mount;
use testcontainers::ImageExt;

use std::borrow::BorrowMut;
use std::env;
use std::fs::File;

use crate::database::*;
use std::io::Write;
use tokio;
pub mod client;
pub mod database;
pub mod message_handler;
pub mod rest;

use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() {
    init_logger(get_log_path());
    app().await;
}

async fn app() {
    //init_logger();
    log::info!("Starting application");

    let mut conn = database::open_sql_connection();
    create_table_if_not_exist(conn.borrow_mut());
    let mqtt_client = client::MqttClient::new().await;
    let mut mqtt_client2 = mqtt_client.clone();

    let task2 = async move {
        log::info!("Start MQTT Receiver Task 2");
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        let message_handler = message_handler::dummy_mqtt::DummyMqttHandler {};
        mqtt_client2.receive(message_handler).await;
    };

    tokio::join!(mqtt_recevier(conn, mqtt_client), task2, tick(60));
}

async fn mqtt_recevier(conn: mysql::PooledConn, mut mqtt_client: client::MqttClient) {
    log::info!("Start MQTT Receiver Task");
    let message_handler = message_handler::mqtt::MqttMessageHandler::new(conn);
    mqtt_client.receive(message_handler).await;
}

async fn tick(seconds: u64) {
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(seconds)).await;
        log::info!("Tick ");
    }
}

fn get_log_path() -> Option<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let name = env!("CARGO_PKG_NAME");
        return Some(args[1].to_owned() + "/" + name + ".log");
    }
    None
}

fn init_logger(file_path: Option<String>) {
    let format_func = |buf: &mut Formatter, record: &Record| {
        writeln!(
            buf,
            "[{}  {} {:?} Task({:?}) {}.{}] {}",
            chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
            record.level(),
            std::thread::current().id(),
            tokio::task::try_id(),
            record.file().unwrap_or("unknown"),
            record.line().unwrap_or(0),
            record.args()
        )
    };
    if file_path.is_some() {
        let file: Box<File> = Box::new(File::create(file_path.unwrap()).unwrap());
        Builder::from_default_env()
            .target(Target::Pipe(file))
            .format(format_func)
            .init();
    } else {
        Builder::from_default_env()
            .target(Target::Stdout)
            .format(format_func)
            .filter_level(log::LevelFilter::Info)
            .init();
    }
}

use ::function_name::named;
use testcontainers::{
    core::{IntoContainerPort, WaitFor},
    runners::AsyncRunner,
    GenericImage,
};

#[tokio::test]
#[named]
async fn test_mqtt() -> Result<(), i32> {
    println!("start : {}", function_name!());

    let log_filepath = function_name!().to_owned() + ".log";
    init_logger(Some(log_filepath));

    log::info!("AAQWE");

    let mqtt = GenericImage::new("eclipse-mosquitto", "latest")
        .with_mapped_port(1883, 1883.tcp())
        .with_network("mosquitto_default")
        .with_mount(Mount::bind_mount(
            "/home/rafal/workspace/mosquitto/broker/mosquitto/config",
            "/mosquitto/config",
        ))
        .with_container_name("mqtt")
        //.with_wait_for(WaitFor::message_on_stdout("Ready to accept connections"))
        .start()
        .await
        .unwrap();
    
    let sql = GenericImage::new("mysql", "latest")
    .with_mapped_port(3306, 3306.tcp())
        .with_env_var("MYSQL_ROOT_PASSWORD", "strong_password")
        .with_network("mosquitto_default")
        .with_container_name("database")
        .start()
        .await
        .unwrap();

    log::info!("Start");
    
    tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;

    tokio::select! {
       _ = app() => {}
       _ = tokio::time::sleep(tokio::time::Duration::from_secs(10)) => { log::error!("Tineout")}
    }
    /*let  mqtt_client2 = mqtt_client.clone();

    let token = CancellationToken::new();
    let sut_token = token.clone();

    log::info!(" start");
    println!("AAA");
    let test_case = async move {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        println!(" test_case");
        mqtt_client2.send().await;
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        println!("cancel token");
        token.cancel();
    };

    let sut = async move {
        let message_handler = message_handler::dummy_mqtt::DummyMqttHandler {};

        tokio::select! {
            _ = sut_token.cancelled() => {}
            _ = mqtt_client.receive(message_handler) => {}
            _ = tokio::time::sleep(tokio::time::Duration::from_secs(10)) => {}
        }

        println!(" test_case2 1");
    };

    tokio::join!(test_case, sut);
    log::info!(" after join");*/
    Ok(())
}
