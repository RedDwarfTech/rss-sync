#![allow(non_upper_case_globals)]
extern crate openssl;
#[macro_use]
extern crate diesel;

use crate::cruise::channel::rss_channel::fetch_channel_article;
use crate::cruise::models::appenum::celery_opt::CeleryOpt;
use cruise::celery::celery_init::init_impl;
use cruise::sched::scheduler::check_tpl_task;
use env_logger::Env;
use structopt::StructOpt;

pub mod common;
pub mod cruise;
pub mod model;
pub mod service;

#[tokio::main]
async fn main() {
    handle_task().await;
}

async fn handle_task() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let opt = CeleryOpt::from_args();
    match &opt {
        CeleryOpt::Consume => {
            let _result = init_impl(&opt).await;
        }
        CeleryOpt::Produce { tasks: _ } => {
            check_tpl_task(&opt).await;
        }
    }
}
