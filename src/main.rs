#![allow(non_upper_case_globals)]
extern crate openssl;

use env_logger::Env;
use log::{error, info};
use rss::sched::scheduler::check_tpl_task;
use crate::rss::{celery::celery_init::init_impl, models::appenum::celery_opt::CeleryOpt};

pub mod rss;

#[tokio::main]
async fn main() {
    env_logger::from_env(Env::default().default_filter_or("info")).init();
    check_tpl_task().await;
}

async fn test_mulit_thread() {
    let mut thread_list = vec![];
    for i in 0..10 {
        let thandle = tokio::spawn(async move {
            
        });
        thread_list.push(thandle);
    }
    for handle in thread_list {
        handle.await.unwrap();
    }
}