pub use crate::client;
use crate::database;
use crate::message_handler::MessageHandler;
use crate::types;
use mysql::serde_json;
use std::sync::Arc;

pub struct MqttMessageHandler<T: database::AsyncQuerryDropable + Send + Sync> {
    sql: T,
    error_tx: tokio::sync::mpsc::Sender<String>,
    shared_data: Arc<tokio::sync::RwLock<types::shared_data::Data>>,
}

impl<T: database::AsyncQuerryDropable + Send + Sync> MqttMessageHandler<T> {
    pub fn new(
        sql: T,
        error_tx: tokio::sync::mpsc::Sender<String>,
        shared_data: Arc<tokio::sync::RwLock<types::shared_data::Data>>,
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

impl<T: database::AsyncQuerryDropable + Send + Sync> MessageHandler<paho_mqtt::Message>
    for MqttMessageHandler<T>
{
    async fn handle_message(
        &mut self,
        msg: paho_mqtt::Message,
        _client: &(impl client::Client<paho_mqtt::Message> + Send + Sync),
    ) {
        log::info!("Received message: {:?}", msg);

        let result = match msg.topic() {
            "temperature" => self.handle_message_internal_multisensor(msg).await,
            "wheather" => self.handle_message_internal_wheater(msg).await,
            "sun" => self.handle_message_internal_sun(msg).await,
            _ => Err(String::from("handler for topic not found")),
        };

        match result {
            Ok(()) => {}
            Err(error) => {
                self.error_tx.send(error).await.unwrap();
            }
        }
    }
}

impl<T: database::AsyncQuerryDropable + Send + Sync> MqttMessageHandler<T> {
    async fn handle_message_internal_multisensor(
        &mut self,
        msg: paho_mqtt::Message,
    ) -> Result<(), String> {
        let parsed =
            serde_json::from_str::<types::json_multisensor::Root>(msg.payload_str().as_ref());
        match parsed {
            Ok(parsed) => {
                log::info!("Parsed: {:?}", parsed);

                let temperature = Self::calculate_temperature(parsed);
                let formatted_date_time =
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
                self.shared_data.write().await.temperature = temperature;
                self.sql
                    .drop_query(temperature, &formatted_date_time)
                    .await
                    .unwrap();
                Ok(())
            }
            Err(..) => {
                log::error!("Error parsing message: {:?}", msg.payload_str());
                Err(String::from("Parsing error"))
            }
        }
    }

    async fn handle_message_internal_wheater(&self, msg: paho_mqtt::Message) -> Result<(), String> {
        let parsed = serde_json::from_str::<types::json_wheather::Root>(msg.payload_str().as_ref());
        match parsed {
            Ok(parsed) => {
                log::info!("Parsed: {:?}", parsed);
                self.shared_data.write().await.clouds_forecast = parsed.wheather;
                Ok(())
            }
            Err(..) => {
                log::error!("Error parsing message: {:?}", msg.payload_str());
                Err(String::from("Parsing error"))
            }
        }
    }

    async fn handle_message_internal_sun(&self, msg: paho_mqtt::Message) -> Result<(), String> {
        let parsed = serde_json::from_str::<types::json_sun::Root>(msg.payload_str().as_ref());
        match parsed {
            Ok(parsed) => {
                log::info!("Parsed: {:?}", parsed);
                self.shared_data.write().await.sun = parsed.sun;
                Ok(())
            }
            Err(..) => {
                log::error!("Error parsing message: {:?}", msg.payload_str());
                Err(String::from("Parsing error"))
            }
        }
    }

    fn calculate_temperature(parsed: types::json_multisensor::Root) -> f32 {
        let mut temperature = (parsed.multi_sensor.sensors[0].value / 100) as f32;
        temperature += (parsed.multi_sensor.sensors[0].value as f32 - temperature * 100.0) / 100.0;
        temperature
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::database::AsyncQuerryDropbaleWrapper;

    #[tokio::test]
    async fn should_push_msg_on_error_channel_when_invalid_message() {
        let querry_dropable_mock = database::MockAsyncQuerryDropable::new();
        let mut error_channel = tokio::sync::mpsc::channel::<String>(3);
        let shared_data = Arc::new(tokio::sync::RwLock::new(types::shared_data::Data::new()));

        let mut handler =
            MqttMessageHandler::new(querry_dropable_mock, error_channel.0, shared_data);
        let msg = paho_mqtt::Message::new("temperature", vec![], paho_mqtt::QOS_0);

        let client = client::MockClient::<paho_mqtt::Message>::new();

        handler.handle_message(msg, &client).await;
        assert_eq!(error_channel.1.recv().await.unwrap(), "Parsing error");
    }

    #[tokio::test]
    async fn should_drop_querry() {
        let mut querry_dropable_mock = database::MockQuerryDropable::new();
        querry_dropable_mock
            .expect_drop_query()
            .returning(|temperature, _datetime| assert_eq!(temperature, 21.37));
        let wrapped_querry_dropable_mock = AsyncQuerryDropbaleWrapper::new(Arc::new(
            tokio::sync::Mutex::new(querry_dropable_mock),
        ));
        let error_channel = tokio::sync::mpsc::channel::<String>(3);
        let shared_data = Arc::new(tokio::sync::RwLock::new(types::shared_data::Data::new()));
        let mut handler =
            MqttMessageHandler::new(wrapped_querry_dropable_mock, error_channel.0, shared_data);

        let json_msg = "{\"multiSensor\": {\"sensors\": [{\"type\": \"temperature\", \"id\": 0, \"value\": 2137, \"trend\": 2, \"state\": 2, \"elapsedTimeS\": -1}]}}";
        let msg = paho_mqtt::Message::new("temperature", json_msg, paho_mqtt::QOS_0);

        let client = client::MockClient::<paho_mqtt::Message>::new();

        handler.handle_message(msg, &client).await;
    }

    #[tokio::test]
    async fn should_save_forcast_in_shared_data() {
        let mut querry_dropable_mock = database::MockQuerryDropable::new();
        querry_dropable_mock
            .expect_drop_query()
            .returning(|temperature, _datetime| assert_eq!(temperature, 21.37));
        let wrapped_querry_dropable_mock = AsyncQuerryDropbaleWrapper::new(Arc::new(
            tokio::sync::Mutex::new(querry_dropable_mock),
        ));
        let error_channel = tokio::sync::mpsc::channel::<String>(3);
        let shared_data = Arc::new(tokio::sync::RwLock::new(types::shared_data::Data::new()));

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
                    types::json_wheather::DtClouds {
                        dt: 1744452000,
                        cloud: 96,
                    },
                    types::json_wheather::DtClouds {
                        dt: 1744455600,
                        cloud: 96,
                    },
                ]
        );
    }

    #[tokio::test]
    async fn should_save_sun_in_shared_data() {
        let querry_dropable_mock = database::MockQuerryDropable::new();
        let wrapped_querry_dropable_mock = AsyncQuerryDropbaleWrapper::new(Arc::new(
            tokio::sync::Mutex::new(querry_dropable_mock),
        ));
        let error_channel = tokio::sync::mpsc::channel::<String>(3);
        let shared_data = Arc::new(tokio::sync::RwLock::new(types::shared_data::Data::new()));
        let mut handler = MqttMessageHandler::new(
            wrapped_querry_dropable_mock,
            error_channel.0,
            shared_data.clone(),
        );

        let json_msg = "{\"sun\": 50}";
        let msg = paho_mqtt::Message::new("sun", json_msg, paho_mqtt::QOS_0);
        let client = client::MockClient::<paho_mqtt::Message>::new();
        handler.handle_message(msg, &client).await;
        assert_eq!(shared_data.read().await.sun, 50);
    }
}
