use futures::{channel::mpsc::Receiver, executor::block_on, stream::StreamExt};
use mqtt::Message;
use paho_mqtt;
use std::time::Duration;

pub trait Client<'a> {
    async fn open(callback: &'a mut dyn FnMut(paho_mqtt::Message)) -> Self;
    async fn send();
    async fn receive(&mut self);
}

pub struct MqttClient<'a> {
    cli: paho_mqtt::AsyncClient,
    conn_opts: paho_mqtt::ConnectOptions,
    callback: &'a mut dyn FnMut(paho_mqtt::Message),
    stream: paho_mqtt::AsyncReceiver<Option<Message>>,
}

impl<'a> Client<'a> for MqttClient<'a> {
    async fn open(callback: &'a mut dyn FnMut(paho_mqtt::Message)) -> Self {
        let mut cli = paho_mqtt::AsyncClient::new("tcp://mosquitto:1883").unwrap();
        let conn_opts = paho_mqtt::ConnectOptionsBuilder::new()
            .keep_alive_interval(Duration::from_secs(20))
            .clean_session(true)
            .finalize();

        let stream = cli.get_stream(10);
        let mut result = cli.connect(conn_opts.clone()).await;
        while result.is_err() {
            result = cli.connect(conn_opts.clone()).await;
            log::info!("Connect result: {:?}", result);
        }

        cli.subscribe("test/topic", mqtt::QOS_0).await.unwrap();

        MqttClient {
            cli: cli,
            conn_opts: conn_opts,
            callback: callback,
            stream: stream,
        }
    }
    async fn send() {}
    async fn receive(&mut self) {
        while let Some(msg) = self.stream.next().await {
            log::debug!("Received message: {:?}", msg);
            match msg {
                Some(msg1) => {
                    (self.callback)(msg1);
                }
                None => {
                    log::warn!("No message received");
                    self.cli.connect(self.conn_opts.clone()).await.unwrap();
                }
            };
        }
    }
}
