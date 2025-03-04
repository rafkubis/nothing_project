use ::function_name::named;
use futures::future::BoxFuture;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::sync::Arc;
use testcontainers::core::logs::consumer::LogConsumer;
use testcontainers::core::Mount;
use testcontainers::ImageExt;
use testcontainers::{
    core::{IntoContainerPort, WaitFor},
    runners::AsyncRunner,
    GenericImage,
};
use tokio;
use tokio::io::AsyncWriteExt;

use crate::app;
pub use crate::logger;
use std::env;

struct MyLogConsumer {
    file: Arc<tokio::sync::Mutex<RefCell<tokio::fs::File>>>,
}

impl MyLogConsumer {
    fn new(file_path: Arc<tokio::sync::Mutex<RefCell<tokio::fs::File>>>) -> Self {
        MyLogConsumer { file: file_path }
    }
}

impl LogConsumer for MyLogConsumer {
    fn accept<'a>(&'a self, record: &'a testcontainers::core::logs::LogFrame) -> BoxFuture<'a, ()> {
        Box::pin(async move {
            self.file
                .lock()
                .await
                .get_mut()
                .borrow_mut()
                .write_all(record.bytes())
                .await
                .unwrap();
        })
    }
}

async fn start_mqtt_container(
    name: &str,
    log_path: String,
) -> testcontainers::ContainerAsync<GenericImage> {

    let log_filepath = log_path.to_owned() + "/" + name + ".log";
    let log_mqtt = Arc::new(tokio::sync::Mutex::new(RefCell::new(
        tokio::fs::File::create(log_filepath).await.unwrap(),
    )));

    let img = GenericImage::new("eclipse-mosquitto", "latest")
        .with_mapped_port(1883, 1883.tcp())
        .with_network("mosquitto_default")
        .with_mount(Mount::bind_mount(
            "/home/rafal/workspace/mosquitto/broker/mosquitto/config",
            "/mosquitto/config",
        ))
        .with_container_name(name)
        .with_log_consumer(MyLogConsumer::new(log_mqtt))
        .start()
        .await
        .unwrap();

    img
}

async fn start_mysql_container(
    name: &str,
    log_path: String,
) -> testcontainers::ContainerAsync<GenericImage> {
    let log_filepath = log_path.to_owned() + "/" + name + ".log";
    let log_sql = Arc::new(tokio::sync::Mutex::new(RefCell::new(
        tokio::fs::File::create(log_filepath).await.unwrap(),
    )));

    let img = GenericImage::new("mysql", "latest")
        .with_mapped_port(3306, 3306.tcp())
        .with_env_var("MYSQL_ROOT_PASSWORD", "strong_password")
        .with_network("mosquitto_default")
        .with_container_name(name)
        .with_log_consumer(MyLogConsumer::new(log_sql))
        .start()
        .await
        .unwrap();

    img
}

#[tokio::test]
#[named]
async fn test_mqtt() -> Result<(), i32> {
    let build_dir = env::var("CARGO_TARGET_DIR").expect("CARGO_TARGET_DIR not set");
    println!("build_dir : {}", build_dir);
    println!("start : {}", function_name!());

    let path = build_dir.to_owned() + "/integration_test/" + function_name!();
    tokio::fs::DirBuilder::new()
        .recursive(true)
        .create(path.clone())
        .await
        .unwrap();
    let log_filepath = path.clone() + "/sut.log";
    logger::init_logger(Some(log_filepath));

    let _containers = tokio::join!(
        start_mysql_container("database", path.clone()),
        start_mqtt_container("mqtt", path.clone())
    );

    log::info!("Start");

    tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;

    tokio::select! {
       _ = app::app() => {}
       _ = tokio::time::sleep(tokio::time::Duration::from_secs(10)) => { log::error!("Tineout")}
    }
    /*let  mqtt_client2 = mqtt_client.clone();

        let token = CancellationToken::new();
        let sut_token = token.clone();

        log::info!(" start");
        println!("AAA");
        let test_case = async move {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            println!(" test_case");
            mqtt_client2.send().await;
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            println!("cancel token");
            token.cancel();
        };

        let sut = async move {
            let message_handler = message_handler::dummy_mqtt::DummyMqttHandler {};
    LogConsumer
                _ = sut_token.cancelled() => {}
                _ = mqtt_client.receive(message_handler) => {}
                _ = tokio::time::sleep(tokio::time::Duration::from_secs(10)) => {}
            }

            println!(" test_case2 1");
        };

        tokio::join!(test_case, sut);
        log::info!(" after join");*/

    Ok(())
}
