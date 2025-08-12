extern crate paho_mqtt as mqtt;
pub use crate::client;
use crate::client::Receiver;
use crate::client::*;
pub use crate::database;
pub use crate::forcast_provider;
pub use crate::logic;
pub use crate::message_handler;
pub use crate::types;
use std::sync::Arc;
use tokio;
use tonic;

tonic::include_proto!("hello");

macro_rules! spawn {
    ($x:expr) => {
        tokio::spawn($x);
    };
    ($e:expr, $($y:expr), *) => {

        spawn!($e);
        spawn!($($y),+);
    }
}

pub async fn app() {
    log::info!("Starting application");
    let shared_data = Arc::new(tokio::sync::RwLock::new(types::shared_data::Data::new()));
    let (mqtt_client, mqtt_client2) = create_mqtt_clients().await;

    let (error_channel_tx, error_channel_rx) = tokio::sync::mpsc::channel::<String>(3);

    let task2 = async move {
        log::info!("Start MQTT Receiver Task 2");
        let message_handler = message_handler::dummy_mqtt::DummyMqttHandler {};
        let mut receiver = mqtt_client2.get_receiver();
        receiver.receive(message_handler).await;
    };

    spawn!(tick(2), tick(1), tick(3));

    let driver_task_handle = tokio::spawn(driver_task(shared_data.clone()));
    let task2_handle = tokio::spawn(task2);
    let mqtt_reciver_handle = tokio::spawn(mqtt_recevier(
        mqtt_client,
        error_channel_tx.clone(),
        shared_data.clone(),
    ));
    let handle_erros_handle = tokio::spawn(handle_errors(error_channel_rx));
    let tonic_task_handle = tokio::spawn(tonic_task());

    _ = tokio::join!(
        mqtt_reciver_handle,
        handle_erros_handle,
        task2_handle,
        driver_task_handle,
        tonic_task_handle
    );
}

pub async fn create_mqtt_clients() -> (client::MqttClient, client::MqttClient) {
    log::info!("Creating MQTT clients");
    let mqtt_client = client::MqttClient::new();
    let mqtt_client2 = client::MqttClient::new();

    log::info!("Connecting to MQTT broker");
    mqtt_client.connect().await;
    mqtt_client2.connect().await;

    let subscribed = async move {
        mqtt_client.subscribe("temperature").await;
        mqtt_client.subscribe("wheather").await;
        mqtt_client.subscribe("sun").await;
        mqtt_client
    };

    let subscribed2 = async move {
        mqtt_client2.subscribe("temperature").await;
        mqtt_client2
    };

    log::info!("Subscribing to topics");
    tokio::join!(subscribed, subscribed2)
}

pub async fn mqtt_recevier(
    mqtt_client: client::MqttClient,
    error_channel_tx: tokio::sync::mpsc::Sender<String>,
    shared_data: Arc<tokio::sync::RwLock<types::shared_data::Data>>,
) {
    log::info!("Start MQTT Receiver Task");
    let conn = tokio::task::spawn_blocking(|| {
        Arc::new(tokio::sync::Mutex::new(database::MySqlQuerryDropbale::new()))
    })
    .await
    .unwrap();

    let async_conn = database::AsyncQuerryDropbaleWrapper::new(conn);
    let message_handler =
        message_handler::mqtt::MqttMessageHandler::new(async_conn, error_channel_tx, shared_data);
    let mut receiver = mqtt_client.get_receiver();
    receiver.receive(message_handler).await;
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

pub async fn create_mqtt_client_without_subscriptions() -> client::MqttClient {
    let mqtt_client = client::MqttClient::new();
    mqtt_client.connect().await;
    mqtt_client
}

pub async fn driver_task(shared_data: Arc<tokio::sync::RwLock<types::shared_data::Data>>) {
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    let forcast_provider = forcast_provider::ForecastProvider::new(shared_data.clone());

    loop {
        let forcast = forcast_provider.get().await;
        if forcast.is_none() {
            log::warn!("No forecast data");
        } else {
            let data = shared_data.read().await;
            let result =
                logic::should_stop(data.temperature, data.sun as i64, forcast.unwrap()).to_string();
            log::info!("Result: {}", result);
            drop(data);

            let msg = paho_mqtt::Message::new("driver", result, paho_mqtt::QOS_2);
            let mqtt_client = create_mqtt_client_without_subscriptions().await;
            mqtt_client.get_sender().send(msg).await;
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
    }
}

pub async fn tonic_task() {
    log::info!("Starting Tonic");
    let mut channel = tonic::transport::Channel::from_static("http://[::1]:50051")
        .connect()
        .await;

    while channel.is_err() {
        log::warn!("Tonic channel not ready, retrying...");
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        channel = tonic::transport::Channel::from_static("http://[::1]:50051")
            .connect()
            .await;
    }

    let mut cli = hellower_client::HellowerClient::new(channel.unwrap());

    let mut req = Request::default();
    req.greet.clone_from(&String::from("source"));
    log::info!("Tonic request: {:?}", req);

    match cli.say_hallo(req).await {
        Ok(resp) => {
            log::info!("Tonic response: {:?}", resp);
        }
        Err(e) => {
            log::error!("Tonic error: {:?}", e);
        }
    };
}
