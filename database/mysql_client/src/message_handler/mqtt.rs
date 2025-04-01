pub use crate::client;
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

use async_trait::async_trait;
use mockall::predicate::*;
use mockall::*;

#[tokio::test]
async fn test() {
    let mock = database::MockQuerryDropable::new();
    let wrapped_mock = Arc::new(tokio::sync::Mutex::new(mock));
    let mut handler = MqttMessageHandler::new(wrapped_mock);
    let msg = paho_mqtt::Message::new("topic", vec![], paho_mqtt::QOS_0);

    /*   mock!{
     pub C{}

      #[async_trait]
       impl client::Client<paho_mqtt::Message> for C{
           async  fn new() -> Self;
             async fn send(&self, str: &str);
           async fn receive<T: MessageHandler<paho_mqtt::Message> + Send + Sync + 'static>(&self, handler: T);
       }
    }*/

    //  handler.handle_message(msg, client)

    //  assert_eq!(handler.handle_message(msg).await, Err(String::from("Parsing errorp")));
}
