use log::error;
use redis::{ Client, AsyncCommands};
use rust_wheel::config::{initial::get_config};

pub async fn send_article_to_stream(stream_key: &str){
    let config_redis_string = get_config("redisConnectionStr");
    let redis_con_string: &str = config_redis_string.as_str();
    let redis_client = Client::open(redis_con_string);
    match redis_client {
        Ok(client) => {
            let connection = client.get_async_connection().await;
            match connection {
                Ok(mut conn) => {
                    let result = conn.xadd::<&str, &str, &str, &str, String>(stream_key, "*", &[("name", "name-02"), ("title", "title 02")]).await;
                    match result {
                        Ok(_) => {

                        },
                        Err(_) => {
                            error!("Couldn't send to redis");
                        },
                    }
                },
                Err(e) => {
                    error!("Couldn't connect to redis,{}",e);
                },
            }
        },
        Err(e) => {
            error!("Couldn't get redis client,{}",e);
        },
    }
}
