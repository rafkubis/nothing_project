pub use crate::message_handler;
use futures::stream::StreamExt;
use std::time::Duration;

pub trait Client<Msg: 'static> {
    fn connect(&self) -> impl std::future::Future<Output = ()>;
    //   fn send(&self, msg: Msg) -> impl std::future::Future<Output = ()> + Send + Sync;
    /*   fn receive<T: message_handler::MessageHandler<Msg> + Send + Sync + 'static>(
        &mut self,
        handler: T,
    ) -> impl std::future::Future<Output = ()> + Send + Sync;*/
    fn get_sender(&self) -> impl Sender<Msg> + Send + Sync + 'static;
    fn get_receiver(&self) -> impl Receiver<Msg> + Send + Sync + 'static;
}

pub trait Sender<Msg: 'static> {
    fn send(&self, msg: Msg) -> impl std::future::Future<Output = ()> + Send + Sync;
}

pub trait Receiver<Msg: 'static> {
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

pub struct MqttSender {
    cli: paho_mqtt::AsyncClient,
}

pub struct MqttReceiver {
    stream: paho_mqtt::AsyncReceiver<Option<paho_mqtt::Message>>,
}

impl MqttSender {
    pub fn new(cli: paho_mqtt::AsyncClient) -> Self {
        Self { cli }
    }
}

impl MqttReceiver {
    pub fn new(stream: paho_mqtt::AsyncReceiver<Option<paho_mqtt::Message>>) -> Self {
        Self { stream }
    }
}

impl Sender<paho_mqtt::Message> for MqttSender {
    async fn send(&self, msg: paho_mqtt::Message) {
        log::debug!("Sending message {}", msg);
        if let Err(err) = self.cli.publish(msg).await {
            log::error!("{}", err);
        }
    }
}

impl Receiver<paho_mqtt::Message> for MqttReceiver {
    async fn receive<
        T: message_handler::MessageHandler<paho_mqtt::Message> + Send + Sync + 'static,
    >(
        &mut self,
        mut handler: T,
    ) {
        while let Some(msg) = self.stream.next().await {
            log::debug!("Received message: {:?}", msg);
            match msg {
                Some(msg1) => {
                    handler.handle_message(msg1).await;
                }
                None => {
                    log::warn!("No message received");
                }
            };
        }
    }
}

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
        let result = self.cli.connect(self.conn_opts.clone()).await;
        log::info!("Connect result: {:?}", result);
    }

    /*async fn receive<T: message_handler::MessageHandler<paho_mqtt::Message> + Send + Sync>(
        &mut self,
        mut handler: T,
    ) {
        while let Some(msg) = self.stream.next().await {
            log::debug!("Received message: {:?}", msg);
            match msg {
                Some(msg1) => {
                    handler.handle_message(msg1).await;
                }
                None => {
                    log::warn!("No message received");
                    //    let _ = self.cli.connect(self.conn_opts.clone()).await.unwrap();
                }
            };
        }
    }*/

    fn get_sender(&self) -> impl Sender<paho_mqtt::Message> + Send + Sync + 'static {
        MqttSender::new(self.cli.clone())
    }

    fn get_receiver(&self) -> impl Receiver<paho_mqtt::Message> + Send + Sync + 'static {
        MqttReceiver::new(self.stream.clone())
    }
}
