pub use crate::message_handler;
use async_trait::async_trait;
use futures::stream::StreamExt;
use mockall;
use paho_mqtt;
use std::time::Duration;

pub trait Client<Msg> {
    fn new() -> impl std::future::Future<Output = Self>;
    fn send(&self, str: &str) -> impl std::future::Future<Output = ()> + Send + Sync;
    fn receive<T: message_handler::MessageHandler<Msg> + Send + Sync>(
        &mut self,
        handler: T,
    ) -> impl std::future::Future<Output = ()> + Send + Sync;
}

#[derive(Clone)]
pub struct MqttClient {
    cli: paho_mqtt::AsyncClient,
    conn_opts: paho_mqtt::ConnectOptions,
    stream: paho_mqtt::AsyncReceiver<Option<paho_mqtt::Message>>,
}

//unsafe impl Send for MqttClient {}
//unsafe impl Sync for MqttClient {}

impl Client<paho_mqtt::Message> for MqttClient {
    async fn new() -> Self {
        let mut cli = paho_mqtt::AsyncClient::new("tcp://mqtt:1883").unwrap();
        let conn_opts = paho_mqtt::ConnectOptionsBuilder::new()
            .keep_alive_interval(Duration::from_secs(20))
            .clean_session(true)
            .user_name("app")
            .password("test_app")
            .finalize();

        let stream = cli.get_stream(10);
        let mut result = cli.connect(conn_opts.clone()).await;
        while result.is_err() {
            result = cli.connect(conn_opts.clone()).await;
            log::info!("Connect result: {:?}", result);
        }

        cli.subscribe("test/topic", paho_mqtt::QOS_0).await.unwrap();

        MqttClient {
            cli: cli,
            conn_opts: conn_opts,
            stream: stream,
        }
    }

    async fn send(&self, str: &str) {
        log::info!("Sending message {}", str);
        let msg = paho_mqtt::Message::new("test/topic", str, paho_mqtt::QOS_0);
        self.cli.publish(msg).await.unwrap();
    }

    async fn receive<T: message_handler::MessageHandler<paho_mqtt::Message> + Send + Sync>(
        &mut self,
        mut handler: T,
    ) {
        while let Some(msg) = self.stream.next().await {
            log::debug!("Received message: {:?}", msg);
            match msg {
                Some(msg1) => {
                    handler.handle_message(msg1, self).await;
                }
                None => {
                    log::warn!("No message received");
                    let _ = self.cli.connect(self.conn_opts.clone()).await.unwrap();
                }
            };
        }
    }
}
