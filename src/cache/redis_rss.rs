use log::error;
use redis::{ Client, Commands, AsyncCommands};
use rust_wheel::config::{initial::get_config, cache::redis_util::get_list_size};

pub async fn async_send_article_to_stream(stream_key: &str){
    let config_redis_string = get_config("REDIS_ADDR");
    let redis_con_string: &str = config_redis_string.as_str();
    let redis_client = Client::open(redis_con_string);
    match redis_client {
        Ok(client) => {
            let connection = client.get_async_connection().await;
            match connection {
                Ok(mut conn) => {
                    let result = conn.xadd::<&str, &str, &str, &str, String>(stream_key, "*", &[("id", "2"), ("sub_source_id", "3")]).await;
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

///
/// the async will take too much memory
/// which lead to memory overflow
/// 
pub fn send_article_to_stream(stream_key: &str){
    let config_redis_string = get_config("REDIS_ADDR");
    let redis_con_string: &str = config_redis_string.as_str();
    let redis_client = Client::open(redis_con_string);
    match redis_client {
        Ok(client) => {
            let connection = client.get_connection();
            match connection {
                Ok(mut conn) => {
                    let result = conn.xadd::<&str, &str, &str, &str, String>(stream_key, "*", &[("id", "2"), ("sub_source_id", "3")]);
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


pub fn get_task_count() -> usize {
    return get_list_size("celery").unwrap();
}
