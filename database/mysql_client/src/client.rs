pub use crate::message_handler;
use async_trait::async_trait;
use futures::stream::StreamExt;
use mockall::*;
use std::time::Duration;

#[async_trait]
#[automock]
pub trait Client<Msg: 'static> {
    fn connect(&self) -> impl std::future::Future<Output = ()>;
    fn send(&self, msg: Msg) -> impl std::future::Future<Output = ()> + Send + Sync;
    fn receive<T: message_handler::MessageHandler<Msg> + Send + Sync + 'static>(
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

impl MqttClient {
    pub async fn subscribe(&self, topic: &str) {
        let token: paho_mqtt::Token = self.cli.subscribe(topic, paho_mqtt::QOS_0);
        let result = token.await;
        log::info!("Subscribe to {} result: {:?}", topic, result);
    }
}

//unsafe impl Send for MqttClient {}
//unsafe impl Sync for MqttClient {}

impl MqttClient {
    pub fn new() -> Self {
        let mut cli = paho_mqtt::AsyncClient::new("tcp://mqtt:1883").unwrap();
        let conn_opts = paho_mqtt::ConnectOptionsBuilder::new()
            .keep_alive_interval(Duration::from_secs(20))
            .clean_session(true)
            .user_name("app")
            .password("test_app")
            .finalize();

        let stream = cli.get_stream(10);

        Self {
            cli: cli,
            conn_opts: conn_opts,
            stream: stream,
        }
    }
}

impl Client<paho_mqtt::Message> for MqttClient {
    async fn connect(&self) {
        self.cli.connect(self.conn_opts.clone()).await.unwrap();
    }

    async fn send(&self, massage: paho_mqtt::Message) {
        log::debug!("Sending message {}", massage);
        if let Err(err) = self.cli.publish(massage).await {
            log::error!("{}", err);
        }
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
                    //    let _ = self.cli.connect(self.conn_opts.clone()).await.unwrap();
                }
            };
        }
    }
}
