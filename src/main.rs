#![allow(non_upper_case_globals)]
extern crate openssl;
#[macro_use]
extern crate diesel;

use crate::cruise::models::appenum::celery_opt::CeleryOpt;
use cruise::celery::celery_init::init_impl;
use cruise::sched::scheduler::check_tpl_task;
use log::error;
use std::thread;

pub mod cache;
pub mod common;
pub mod cruise;
pub mod model;
pub mod service;

#[tokio::main]
async fn main() {
    let log_result = log4rs::init_file("log4rs.yaml", Default::default());
    if let Err(e) = log_result {
        error!("init log failed, {}", e);
    }
    let produce_thread = thread::spawn(|| {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let tasks = vec!["add".to_string()];
            handle_task(&CeleryOpt::Produce { tasks }).await;
        });
    });

    let consume_thread = thread::spawn(|| {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            handle_task(&CeleryOpt::Consume).await;
        });
    });

    produce_thread.join().unwrap();
    consume_thread.join().unwrap();
}

async fn handle_task(opt: &CeleryOpt) {
    match &opt {
        CeleryOpt::Consume => {
            let result = init_impl(&opt).await;
            if let Err(e) = result {
                error!("start consume celery failed, {}", e);
            }
        }
        CeleryOpt::Produce { tasks: _ } => {
            check_tpl_task(&opt).await;
        }
    }
}
