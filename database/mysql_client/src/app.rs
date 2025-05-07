extern crate paho_mqtt as mqtt;
pub use crate::client;
use crate::client::Client;
pub use crate::database;
pub use crate::message_handler;
use std::sync::Arc;
use tokio;

pub async fn app() {
    log::info!("Starting application");
    let shared_data = Arc::new(tokio::sync::RwLock::new(
        message_handler::shared_data::Data {
            clouds_forecast: vec![],
        },
    ));
    let mqtt_client = create_mqtt_client().await;
    let mut mqtt_client2 = create_mqtt_client().await;

    let (error_channel_tx, error_channel_rx) = tokio::sync::mpsc::channel::<String>(3);

    let task2 = async move {
        log::info!("Start MQTT Receiver Task 2");
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        let message_handler = message_handler::dummy_mqtt::DummyMqttHandler {};
        mqtt_client2.receive(message_handler).await;
    };
    let tick_1s_handle = tokio::spawn(tick(1));
    let tick_2s_handle = tokio::spawn(tick(2));
    let task_2_handle = tokio::spawn(task2);
    let mqtt_receiver_handle = tokio::spawn(mqtt_recevier(
        mqtt_client,
        error_channel_tx.clone(),
        shared_data,
    ));
    let _error_handle = tokio::spawn(handle_errors(error_channel_rx));

    let _ = tokio::join!(
        tick_1s_handle,
        tick_2s_handle,
        task_2_handle,
        mqtt_receiver_handle
    );
}

pub async fn mqtt_recevier(
    mut mqtt_client: client::MqttClient,
    error_channel_tx: tokio::sync::mpsc::Sender<String>,
    shared_data: Arc<tokio::sync::RwLock<message_handler::shared_data::Data>>,
) {
    log::info!("Start MQTT Receiver Task");
    let conn = Arc::new(tokio::sync::Mutex::new(database::MySqlQuerryDropbale::new()));
    let message_handler =
        message_handler::mqtt::MqttMessageHandler::new(conn, error_channel_tx, shared_data);
    mqtt_client.receive(message_handler).await;
}

pub async fn tick(seconds: u64) {
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(seconds)).await;
        log::info!("Tick {} s", seconds);
    }
}

pub async fn handle_errors(mut rx: tokio::sync::mpsc::Receiver<String>) {
    while let Some(err) = rx.recv().await {
        log::error!("{err}");
    }
    log::warn!("channel is closed");
}

pub async fn create_mqtt_client() -> client::MqttClient {
    let mqtt_client = client::MqttClient::new();
    mqtt_client.connect().await;
    mqtt_client.subscribe("temperature").await;
    mqtt_client.subscribe("wheather").await;
    mqtt_client
}
