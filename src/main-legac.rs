#![allow(non_upper_case_globals)]

use env_logger::Env;
use rss::celery::celery_init::init_impl;
use log::{info, error};

pub mod rss;

use celery::prelude::*;

#[celery::task]
fn add(x: i32, y: i32) -> TaskResult<i32> {
    Ok(x + y)
}

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let redis_addr = std::env::var("REDIS_ADDR").unwrap_or_else(|_| "redis://127.0.0.1:6379/".into());
    let my_app = celery::app!(
        broker = RedisBroker { redis_addr },
        tasks = [add],
        task_routes = [
            "*" => "celery",
        ],
    ).await;
   // my_app.send_task(add::new(1, 2)).await?;

}
