#![allow(non_upper_case_globals)]
extern crate openssl;
#[macro_use]
extern crate diesel;

use crate::cruise::models::appenum::celery_opt::CeleryOpt;
use common::monitor::profile_controller;
use cruise::celery::celery_init::init_impl;
use cruise::sched::scheduler::check_tpl_task;
use log::error;
use rust_wheel::config::app::app_conf_reader::get_app_config;
use std::thread;
use actix_web::{App, HttpServer};

pub mod cache;
pub mod common;
pub mod cruise;
pub mod model;
pub mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
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

    let port: u16 = get_app_config("rsssync.port").parse().unwrap();
    let address = ("0.0.0.0", port);
    HttpServer::new(|| {
        App::new()
            .configure(profile_controller::config)
    })
    .bind(address)?
    .run()
    .await
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
