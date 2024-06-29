use mysql::prelude::Queryable;
use mysql::*;
extern crate paho_mqtt as mqtt;
use std::time::Duration;
use chrono;

pub mod rest;


fn main() {
    let mut conn = open_sql_connection();
    create_table_if_not_exist(&mut conn);

    let cli = mqtt::Client::new("tcp://mosquitto:1883").unwrap();

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
    receiver.iter().for_each(|msg| match msg {
        None => println!("None"),
        Some(value) => {
            let parsed = serde_json::from_str::<rest::rest::Root>(value.payload_str().as_ref());
            match parsed {
                Ok(parsed) => {
                    println!("Parsed: {:?}", parsed);

                    let mut temperature = (parsed.multiSensor.sensors[0].value / 100) as f32;
                    temperature += (parsed.multiSensor.sensors[0].value as f32 - temperature*100.0) / 100.0;

                    let date_time = chrono::Local::now();
                    let formatted_date_time = date_time.format("%Y-%m-%d %H:%M:%S").to_string();
                    drop_querry(&mut conn, temperature, &formatted_date_time);
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            }
        }
    });

    println!("Hello, world!");
}

fn drop_querry(conn: &mut PooledConn, temperature: f32, datetime: &str) {
    let temperature_str = temperature.to_string();
    let mut querry = "INSERT INTO users (temperature, datetime) VALUES (".to_string();
    querry.push_str(&temperature_str);
    querry.push_str(", '");
    querry.push_str(datetime);
    querry.push_str("')");
    conn.query_drop(querry).unwrap();
}

fn create_table_if_not_exist(conn: &mut PooledConn) {
    conn.query_drop("CREATE DATABASE IF NOT EXISTS test")
        .unwrap();
    conn.query_drop("USE test").unwrap();
    conn.query_drop("CREATE TABLE IF NOT EXISTS users (temperature FLOAT, datetime DATETIME)")
        .unwrap();
}

fn open_sql_connection() -> PooledConn {
    let sql_url = "mysql://root:strong_password@database:3306";
    let pool = Pool::new(sql_url).unwrap();
    dbg!(&pool);
    let conn = pool.get_conn().unwrap();
    dbg!(&conn);
    conn
}
