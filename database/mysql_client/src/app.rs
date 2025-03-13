extern crate paho_mqtt as mqtt;
pub use crate::client;
use crate::client::Client;
pub use crate::database;
use crate::database::*;
pub use crate::message_handler;
pub use crate::rest;
use std::borrow::BorrowMut;
use tokio;

pub async fn app() {
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

    tokio::join!(task2, mqtt_recevier(conn, mqtt_client),  tick(60));
}

pub async fn mqtt_recevier(conn: mysql::PooledConn, mut mqtt_client: client::MqttClient) {
    log::info!("Start MQTT Receiver Task");
    let message_handler = message_handler::mqtt::MqttMessageHandler::new(conn);
    mqtt_client.receive(message_handler).await;
}

pub async fn tick(seconds: u64) {
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(seconds)).await;
        log::info!("Tick ");
    }
}
