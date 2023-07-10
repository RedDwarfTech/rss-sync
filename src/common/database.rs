use diesel::PgConnection;
use rust_wheel::config::db::config;

pub fn get_connection() -> PgConnection{
    let connection = config::connection("CV_DATABASE_URL".to_string());
    return connection;
}