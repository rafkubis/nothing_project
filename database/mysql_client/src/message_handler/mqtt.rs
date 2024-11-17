pub use crate::client;
use crate::database::*;
use crate::message_handler::MessageHandler;
use crate::rest;
use mysql::serde_json;

pub struct MqttMessageHandler {
    sql: mysql::PooledConn,
}

impl MqttMessageHandler {
    pub fn new(sql: mysql::PooledConn) -> Self {
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
        Self::handle_message(msg, &mut self.sql);
        //_client.send().await;
    }
}

impl MqttMessageHandler {
    fn handle_message(msg: paho_mqtt::Message, conn: &mut mysql::PooledConn) {
        let parsed = serde_json::from_str::<rest::Root>(msg.payload_str().as_ref());
        match parsed {
            Ok(parsed) => {
                log::info!("Parsed: {:?}", parsed);

                let temperature = Self::calculate_temperature(parsed);
                let formatted_date_time =
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
                drop_querry(conn, temperature, &formatted_date_time);
                get_table(conn);
            }
            Err(..) => {
                log::error!("Error parsing message: {:?}", msg.payload_str());
            }
        }
    }

    fn calculate_temperature(parsed: rest::Root) -> f32 {
        let mut temperature = (parsed.multi_sensor.sensors[0].value / 100) as f32;
        temperature += (parsed.multi_sensor.sensors[0].value as f32 - temperature * 100.0) / 100.0;
        temperature
    }
}
