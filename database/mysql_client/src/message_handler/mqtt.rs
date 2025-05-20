use super::json_multisensor;
use super::json_wheather;
use super::shared_data;
pub use crate::client;
use crate::database;
use crate::message_handler::MessageHandler;
use mysql::serde_json;
use std::sync::Arc;

pub struct MqttMessageHandler {
    sql: Arc<tokio::sync::Mutex<dyn database::QuerryDropable + Send + Sync>>,
    error_tx: tokio::sync::mpsc::Sender<String>,
    shared_data: Arc<tokio::sync::RwLock<shared_data::Data>>,
}

impl MqttMessageHandler {
    pub fn new(
        sql: Arc<tokio::sync::Mutex<dyn database::QuerryDropable + Send + Sync>>,
        error_tx: tokio::sync::mpsc::Sender<String>,
        shared_data: Arc<tokio::sync::RwLock<shared_data::Data>>,
    ) -> Self {
        MqttMessageHandler {
            sql,
            error_tx,
            shared_data,
        }
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

        let result = match msg.topic() {
            "temperature" => self.handle_message_internal_multisensor(msg).await,
            "wheather" => self.handle_message_internal_wheater(msg).await,
            _ => Err(String::from("handler for topic not found")),
        };

        match result {
            Ok(_) => {}
            Err(error) => {
                self.error_tx.send(error).await.unwrap();
            }
        }
    }
}

impl MqttMessageHandler {
    async fn handle_message_internal_multisensor(
        &mut self,
        msg: paho_mqtt::Message,
    ) -> Result<f32, String> {
        let parsed = serde_json::from_str::<json_multisensor::Root>(msg.payload_str().as_ref());
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

    async fn handle_message_internal_wheater(
        &self,
        msg: paho_mqtt::Message,
    ) -> Result<f32, String> {
        let parsed = serde_json::from_str::<json_wheather::Root>(msg.payload_str().as_ref());
        match parsed {
            Ok(parsed) => {
                log::info!("Parsed: {:?}", parsed);
                let mut shared_data = self.shared_data.write().await;
                shared_data.clouds_forecast = parsed.wheather;
                Ok(1.2)
            }
            Err(..) => {
                log::error!("Error parsing message: {:?}", msg.payload_str());
                Err(String::from("Parsing error"))
            }
        }
    }

    fn calculate_temperature(parsed: json_multisensor::Root) -> f32 {
        let mut temperature = (parsed.multi_sensor.sensors[0].value / 100) as f32;
        temperature += (parsed.multi_sensor.sensors[0].value as f32 - temperature * 100.0) / 100.0;
        temperature
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn should_push_msg_on_error_channel_when_invalid_message() {
        let querry_dropable_mock = database::MockQuerryDropable::new();
        let wrapped_querry_dropable_mock = Arc::new(tokio::sync::Mutex::new(querry_dropable_mock));
        let mut error_channel = tokio::sync::mpsc::channel::<String>(3);
        let shared_data = Arc::new(tokio::sync::RwLock::new(shared_data::Data {
            clouds_forecast: vec![],
        }));

        let mut handler =
            MqttMessageHandler::new(wrapped_querry_dropable_mock, error_channel.0, shared_data);
        let msg = paho_mqtt::Message::new("temperature", vec![], paho_mqtt::QOS_0);

        let client = client::MockClient::<paho_mqtt::Message>::new();

        handler.handle_message(msg, &client).await;
        assert_eq!(error_channel.1.recv().await.unwrap(), "Parsing error");
    }

    #[tokio::test]
    async fn should_drop_querry() {
        let mut querry_dropable_mock = database::MockQuerryDropable::new();
        querry_dropable_mock
            .expect_drop_querry()
            .returning(|temperature, _datetime| assert_eq!(temperature, 21.37));
        let wrapped_querry_dropable_mock = Arc::new(tokio::sync::Mutex::new(querry_dropable_mock));
        let error_channel = tokio::sync::mpsc::channel::<String>(3);
        let shared_data = Arc::new(tokio::sync::RwLock::new(shared_data::Data {
            clouds_forecast: vec![],
        }));
        let mut handler =
            MqttMessageHandler::new(wrapped_querry_dropable_mock, error_channel.0, shared_data);

        let json_msg = "{\"multiSensor\": {\"sensors\": [{\"type\": \"temperature\", \"id\": 0, \"value\": 2137, \"trend\": 2, \"state\": 2, \"elapsedTimeS\": -1}]}}";
        let msg = paho_mqtt::Message::new("temperature", json_msg, paho_mqtt::QOS_0);

        let client = client::MockClient::<paho_mqtt::Message>::new();

        handler.handle_message(msg, &client).await;
    }

    #[tokio::test]
    async fn should_painc() {
        let mut querry_dropable_mock = database::MockQuerryDropable::new();
        querry_dropable_mock
            .expect_drop_querry()
            .returning(|temperature, _datetime| assert_eq!(temperature, 21.37));
        let wrapped_querry_dropable_mock = Arc::new(tokio::sync::Mutex::new(querry_dropable_mock));
        let error_channel = tokio::sync::mpsc::channel::<String>(3);
        let shared_data = Arc::new(tokio::sync::RwLock::new(shared_data::Data {
            clouds_forecast: vec![],
        }));

        let mut handler = MqttMessageHandler::new(
            wrapped_querry_dropable_mock,
            error_channel.0,
            shared_data.clone(),
        );

        let json_msg = "{\"wheather\": [{\"dt\": 1744452000, \"cloud\": 96}, {\"dt\": 1744455600, \"cloud\": 96}]}";
        let msg = paho_mqtt::Message::new("wheather", json_msg, paho_mqtt::QOS_0);
        let client = client::MockClient::<paho_mqtt::Message>::new();

        handler.handle_message(msg, &client).await;

        assert!(
            shared_data.read().await.clouds_forecast
                == vec![
                    json_wheather::DtClouds {
                        dt: 1744452000,
                        cloud: 96,
                    },
                    json_wheather::DtClouds {
                        dt: 1744455600,
                        cloud: 96,
                    },
                ]
        );
    }
}
