use mysql::*;
extern crate paho_mqtt as mqtt;
use std::time::Duration;
use serde::{Deserialize};
/*
#[derive(Debug, Deserialize)]
struct Sensor {
    #[serde(rename = "type")]
    sensor_type: String,
    id: i32,
    value: i32,
    trend: i32,
    state: i32,
    #[serde(rename = "elapsedTimeS")]
    elapsed_time_s: i32,
}

#[derive(Debug, Deserialize)]
struct MultiSensor {
    sensors: Vec<Sensor>,
}

#[derive(Debug, Deserialize)]
struct Root {
    multi_sensor: MultiSensor,
}*/

fn main() {
    /*let sql_url = "mysql://root:strong_password@172.18.11.0:3306";
    let pool = Pool::new(sql_url).unwrap();
    dbg!(&pool);
    let mut conn = pool.get_conn().unwrap();
    dbg!(&conn);

    conn.query_drop("CREATE DATABASE IF NOT EXISTS test").unwrap();
    conn.query_drop("USE test").unwrap();
    // Change to temperature and datetime
    conn.query_drop("CREATE TABLE IF NOT EXISTS users (temperature float, datetime text)").unwrap();

    conn.query_drop("INSERT INTO users (temperature, datetime) VALUES (23.5, '2022-01-01 10:00:00')").unwrap();
    conn.query_drop("INSERT INTO users (temperature, datetime) VALUES (24.2, '2022-01-02 11:00:00')").unwrap();
    conn.query_drop("INSERT INTO users (temperature, datetime) VALUES (22.1, '2022-01-03 12:00:00')").unwrap();*/

    let cli = mqtt::Client::new("tcp://172.17.0.2:1883").unwrap();

    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(20))
        .clean_session(true)
        .finalize();

    if let Err(e) = cli.connect(conn_opts) {
        println!("Unable to connect:\n\t{:?}", e);
    }

    let msg = mqtt::Message::new("test", "Hello world!", 0);
    if let Err(e2) = cli.publish(msg) {
        println!("Unable to publish message:\n\t{:?}", e2);
    }

    let sub: mqtt::ServerResponse = cli.subscribe("test/topic", 0).unwrap();
    dbg!(sub);
    let receiver = cli.start_consuming();
    dbg!(&receiver);
    receiver.iter().for_each(|msg| {        
        println!("Received message: {:?}", msg.expect("AAA").payload_str());
    });

    println!("Hello, world!");
}
