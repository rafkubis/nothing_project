use mockall::automock;
use mysql::prelude::Queryable;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::{self, JoinError};

#[automock]
pub trait QuerryDropable {
    fn drop_query(&mut self, temp: f32, datetime: &str);
}

#[automock]
pub trait AsyncQuerryDropable {
    fn drop_query(
        &mut self,
        temp: f32,
        datetime: &str,
    ) -> impl std::future::Future<Output = Result<(), JoinError>> + Send + Sync;
}

pub struct MySqlQuerryDropbale {
    conn: mysql::PooledConn,
}

pub struct AsyncQuerryDropbaleWrapper {
    querry_dropable: Arc<Mutex<dyn QuerryDropable + Send>>,
}

impl QuerryDropable for MySqlQuerryDropbale {
    fn drop_query(&mut self, temperature: f32, datetime: &str) {
        let querry = std::format!(
            "INSERT INTO users (temperature, datetime) VALUES ('{temperature}', '{datetime}')"
        );
        let result = self.conn.query_drop(querry);
        log::info!("drop_querry result = {:?}", result);
    }
}

impl MySqlQuerryDropbale {
    pub fn new() -> Self {
        let mut conn = Self::open_sql_connection();
        Self::create_table_if_not_exist(&mut conn);
        log::info!("MySqlQuerryDropbale created");
        MySqlQuerryDropbale { conn }
    }

    pub fn create_table_if_not_exist(conn: &mut mysql::PooledConn) {
        conn.query_drop("CREATE DATABASE IF NOT EXISTS test")
            .unwrap();
        conn.query_drop("USE test").unwrap();
        conn.query_drop("CREATE TABLE IF NOT EXISTS users (temperature FLOAT, datetime DATETIME)")
            .unwrap();
    }

    fn open_sql_connection() -> mysql::PooledConn {
        let sql_url = "mysql://root:strong_password@database:3306";
        let pool = mysql::Pool::new(sql_url).unwrap();
        let conn = pool.get_conn().unwrap();
        conn
    }
}

impl AsyncQuerryDropbaleWrapper {
    pub fn new(querry_dropable: Arc<Mutex<dyn QuerryDropable + Send>>) -> Self {
        AsyncQuerryDropbaleWrapper {
            querry_dropable: querry_dropable,
        }
    }
}

impl AsyncQuerryDropable for AsyncQuerryDropbaleWrapper {
    fn drop_query(
        &mut self,
        temp: f32,
        datetime: &str,
    ) -> impl std::future::Future<Output = Result<(), JoinError>> {
        let querry_dropable = self.querry_dropable.clone();
        let dt = datetime.to_string();
        task::spawn_blocking(move || {
            querry_dropable
                .blocking_lock()
                .drop_query(temp, dt.as_str());
        })
    }
}
