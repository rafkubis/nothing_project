#![allow(unused_imports)]
use ::function_name::named;
use futures::future::BoxFuture;
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

use crate::{app, client::Client, client::MqttClient, database, logger};
use mysql::{prelude::Queryable, Row};
use std::env;

#[cfg(test)]
struct MyLogConsumer {
    file: Arc<tokio::sync::Mutex<RefCell<tokio::fs::File>>>,
}

#[cfg(test)]
impl MyLogConsumer {
    fn new(file_path: Arc<tokio::sync::Mutex<RefCell<tokio::fs::File>>>) -> Self {
        MyLogConsumer { file: file_path }
    }
}

#[cfg(test)]
impl LogConsumer for MyLogConsumer {
    fn accept<'a>(&'a self, record: &'a testcontainers::core::logs::LogFrame) -> BoxFuture<'a, ()> {
        Box::pin(async move {
            self.file
                .lock()
                .await
                .get_mut()
                .write_all(record.bytes())
                .await
                .unwrap();
        })
    }
}

#[cfg(test)]
async fn start_mqtt_container(
    name: &str,
    log_path: String,
) -> testcontainers::ContainerAsync<GenericImage> {
    let host_project_pwd = env::var("HOST_PROJECT_PWD").expect("HOST_PROJECT_PWD not set");

    let log_filepath = log_path.to_owned() + "/" + name + ".log";
    let log_mqtt = Arc::new(tokio::sync::Mutex::new(RefCell::new(
        tokio::fs::File::create(log_filepath).await.unwrap(),
    )));
    let ready_msg = "mosquitto version 2.0.20 running";
    let img = GenericImage::new("eclipse-mosquitto", "latest")
        .with_wait_for(WaitFor::message_on_stdout(ready_msg))
        .with_mapped_port(1883, 1883.tcp())
        .with_network("mosquitto_default")
        .with_mount(Mount::bind_mount(
            host_project_pwd + "/broker/mosquitto/",
            "/mosquitto/",
        ))
        .with_container_name(name)
        .with_log_consumer(MyLogConsumer::new(log_mqtt));

    img.start().await.unwrap()
}

#[cfg(test)]
async fn start_mysql_container(
    name: &str,
    log_path: String,
) -> testcontainers::ContainerAsync<GenericImage> {
    let log_filepath = log_path.to_owned() + "/" + name + ".log";
    let log_sql = Arc::new(tokio::sync::Mutex::new(RefCell::new(
        tokio::fs::File::create(log_filepath).await.unwrap(),
    )));
    let ready_msg = "/usr/sbin/mysqld: ready for connections.";
    let img = GenericImage::new("mysql", "latest")
        .with_wait_for(WaitFor::message_on_stderr(ready_msg))
        .with_mapped_port(3306, 3306.tcp())
        .with_env_var("MYSQL_ROOT_PASSWORD", "strong_password")
        .with_network("mosquitto_default")
        .with_container_name(name)
        .with_log_consumer(MyLogConsumer::new(log_sql));

    img.start().await.unwrap()
}

#[cfg(test)]
pub fn open_sql_connection() -> mysql::PooledConn {
    let sql_url = "mysql://root:strong_password@database:3306";
    let pool = mysql::Pool::new(sql_url).unwrap();
    let conn = pool.get_conn().unwrap();
    conn
}

#[tokio::test]
#[named]
async fn test_mqtt() {
    let build_dir = env::var("CARGO_TARGET_DIR").expect("CARGO_TARGET_DIR not set");
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

    tokio::time::sleep(tokio::time::Duration::from_secs(20)).await;

    let test_case = async {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        let client = MqttClient::new().await;
        let message = "{\"multiSensor\": {\"sensors\": [{\"type\": \"temperature\", \"id\": 0, \"value\": 2137, \"trend\": 2, \"state\": 2, \"elapsedTimeS\": -1}]}}";
        client.send(message).await;
        client.send(message).await;
        client.send(message).await;

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        let mut db_connection = open_sql_connection();
        db_connection.query_drop("USE test").unwrap();
        let querry: Vec<Row> = db_connection
            .query::<Row, &str>("select * from users")
            .unwrap();

        let top: &Row = &querry[0];
        let value = top.as_ref(0).unwrap().as_sql(true);

        assert_eq!(querry.len(), 3);
        assert_eq!(value, "'21.37'");
    };

    tokio::select! {
        _ = app::app() => {}
        _ = tokio::time::sleep(tokio::time::Duration::from_secs(30)) =>
        {
            log::error!("Tineout");
            assert!(false);
        }
        _ = test_case => {}
    };
}
