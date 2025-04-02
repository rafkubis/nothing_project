extern crate paho_mqtt as mqtt;
pub use crate::client;
use crate::client::Client;
pub use crate::database;
use crate::database::*;
pub use crate::message_handler;
pub use crate::rest;
use std::borrow::BorrowMut;
use std::sync::Arc;
use tokio;

pub async fn app() {
    log::info!("Starting application");
    let mqtt_client = client::MqttClient::connect().await;
    let mut mqtt_client2 = mqtt_client.clone();

    let task2 = async move {
        log::info!("Start MQTT Receiver Task 2");
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        let message_handler = message_handler::dummy_mqtt::DummyMqttHandler {};
        mqtt_client2.receive(message_handler).await;
    };

    let tick_1s_handle = tokio::spawn(tick(1));
    let tick_2s_handle = tokio::spawn(tick(2));
    let task_2_handle = tokio::spawn(task2);
    let mqtt_receiver_handle = tokio::spawn(mqtt_recevier(mqtt_client));

    tick_1s_handle.await.unwrap();
    tick_2s_handle.await.unwrap();
    task_2_handle.await.unwrap();
    mqtt_receiver_handle.await.unwrap();
}

pub async fn mqtt_recevier(mut mqtt_client: client::MqttClient) {
    log::info!("Start MQTT Receiver Task");
    let conn = Arc::new(tokio::sync::Mutex::new(database::MySqlQuerryDropbale::new()));
    let message_handler = message_handler::mqtt::MqttMessageHandler::new(conn);
    mqtt_client.receive(message_handler).await;
}

pub async fn tick(seconds: u64) {
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(seconds)).await;
        log::info!("Tick {} s", seconds);
    }
}
