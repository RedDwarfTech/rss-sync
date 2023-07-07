#![allow(non_upper_case_globals)]

use env_logger::Env;
use rss::celery::celery_init::init_impl;
use log::{info, error};

pub mod rss;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let r = init_impl().await;
    match r {
        Ok(_) => {
            info!("initial celery success");
        },
        Err(err) => {
            error!("initial celery failed:{}",err);
        },
    }
}
