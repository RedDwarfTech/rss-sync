#![allow(non_upper_case_globals)]
extern crate openssl;
#[macro_use]
extern crate diesel;

use env_logger::Env;
use cruise::celery::celery_init::init_impl;
use cruise::sched::scheduler::check_tpl_task;
use structopt::StructOpt;
use crate::cruise::{ models::appenum::celery_opt::CeleryOpt};
use crate::cruise::channel::rss_channel::fetch_channel_article;

pub mod cruise;
pub mod model;
pub mod service;
pub mod common;

#[tokio::main]
async fn main() {
    //handle_task();
    fetch_channel_article().await;   
}

async fn _handle_task(){
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let opt = CeleryOpt::from_args();
    match &opt {
        CeleryOpt::Consume => {
            let _result = init_impl(&opt).await;
        },
        CeleryOpt::Produce { tasks:_ } => {
            check_tpl_task(&opt).await;
        },
    }
}
