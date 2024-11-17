pub use crate::client;
use crate::message_handler::MessageHandler;

pub struct DummyMqttHandler {}

impl MessageHandler<paho_mqtt::Message> for DummyMqttHandler {
    async fn handle_message(
        &mut self,
        msg: paho_mqtt::Message,
        _client: &(impl client::Client<paho_mqtt::Message> + Send + Sync),
    ) {
        log::info!("handle_message: {}", msg);
    }
}
