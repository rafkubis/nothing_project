use mysql::{prelude::Queryable, Row};

pub fn drop_querry(conn: &mut mysql::PooledConn, temperature: f32, datetime: &str) {
    let temperature_str = temperature.to_string();
    let mut querry = "INSERT INTO users (temperature, datetime) VALUES (".to_string();
    querry.push_str(&temperature_str);
    querry.push_str(", '");
    querry.push_str(&datetime);
    querry.push_str("')");
    conn.query_drop(querry).unwrap();
}

pub fn create_table_if_not_exist(conn: &mut mysql::PooledConn) {
    conn.query_drop("CREATE DATABASE IF NOT EXISTS test")
        .unwrap();
    conn.query_drop("USE test").unwrap();
    conn.query_drop("CREATE TABLE IF NOT EXISTS users (temperature FLOAT, datetime DATETIME)")
        .unwrap();
}

pub fn get_table(conn: &mut mysql::PooledConn) {
    conn.query_first("select * from users")
        .map(|res: Option<Row>| {
            let r = res.unwrap();
            log::info!("get_table {:?}", r.columns());
            log::info!("get_table {:?}", r.len());
            //log::info!("get_table {:?}", r.unwrap());

            let c = r.columns();
            let a: &mysql::Column = &c[0];
            log::info!("c = {:?}", a);
        })
        .unwrap();
}

pub fn open_sql_connection() -> mysql::PooledConn {
    let sql_url = "mysql://root:strong_password@database:3306";
    let pool = mysql::Pool::new(sql_url).unwrap();
    let conn = pool.get_conn().unwrap();
    conn
}
