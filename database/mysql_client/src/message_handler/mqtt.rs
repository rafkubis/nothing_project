pub use crate::client;
use crate::client::Client;
use crate::database;
use crate::message_handler::MessageHandler;
use crate::rest;
use mysql::serde_json;
use std::sync::Arc;

pub struct MqttMessageHandler {
    sql: Arc<tokio::sync::Mutex<dyn database::QuerryDropable + Send + Sync>>,
}

impl MqttMessageHandler {
    pub fn new(sql: Arc<tokio::sync::Mutex<dyn database::QuerryDropable + Send + Sync>>) -> Self {
        MqttMessageHandler { sql }
    }
}

//unsafe impl Send for MqttMessageHandler {}
//unsafe impl Sync for MqttMessageHandler {}

impl MessageHandler<paho_mqtt::Message> for MqttMessageHandler {
    async fn handle_message(
        &mut self,
        msg: paho_mqtt::Message,
        _client: &(impl client::Client<paho_mqtt::Message> + Send + Sync),
    ) {
        log::info!("Received message: {:?}", msg);
        self.handle_message_internal(msg).await.unwrap();
    }
}

impl MqttMessageHandler {
    async fn handle_message_internal(&mut self, msg: paho_mqtt::Message) -> Result<f32, String> {
        let parsed = serde_json::from_str::<rest::Root>(msg.payload_str().as_ref());
        match parsed {
            Ok(parsed) => {
                log::info!("Parsed: {:?}", parsed);

                let temperature = Self::calculate_temperature(parsed);
                let formatted_date_time =
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
                self.sql
                    .lock()
                    .await
                    .drop_querry(temperature, &formatted_date_time);
                Ok(temperature)
            }
            Err(..) => {
                log::error!("Error parsing message: {:?}", msg.payload_str());
                Err(String::from("Parsing error"))
            }
        }
    }

    fn calculate_temperature(parsed: rest::Root) -> f32 {
        let mut temperature = (parsed.multi_sensor.sensors[0].value / 100) as f32;
        temperature += (parsed.multi_sensor.sensors[0].value as f32 - temperature * 100.0) / 100.0;
        temperature
    }
}

#[tokio::test]
#[should_panic]
async fn should_panic_when_invalid_message() {
    let querry_dropable_mock = database::MockQuerryDropable::new();
    let wrapped_querry_dropable_mock = Arc::new(tokio::sync::Mutex::new(querry_dropable_mock));
    let mut handler = MqttMessageHandler::new(wrapped_querry_dropable_mock);
    let msg = paho_mqtt::Message::new("topic", vec![], paho_mqtt::QOS_0);

    let client = client::MockClient::<paho_mqtt::Message>::new();

    handler.handle_message(msg, &client).await;
}

#[tokio::test]
async fn should_drop_querry() {
    let mut querry_dropable_mock = database::MockQuerryDropable::new();
    let _querry_exoectation = querry_dropable_mock
        .expect_drop_querry()
        .returning(|temperature, _datetime| assert_eq!(temperature, 21.37));
    let wrapped_querry_dropable_mock = Arc::new(tokio::sync::Mutex::new(querry_dropable_mock));
    let mut handler = MqttMessageHandler::new(wrapped_querry_dropable_mock);

    let json_msg = "{\"multiSensor\": {\"sensors\": [{\"type\": \"temperature\", \"id\": 0, \"value\": 2137, \"trend\": 2, \"state\": 2, \"elapsedTimeS\": -1}]}}";
    let msg = paho_mqtt::Message::new("topic", json_msg, paho_mqtt::QOS_0);

    let client = client::MockClient::<paho_mqtt::Message>::new();

    handler.handle_message(msg, &client).await;
}
