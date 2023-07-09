#![allow(non_upper_case_globals)]
extern crate openssl;
#[macro_use]
extern crate diesel;

use env_logger::Env;
use rss::{sched::scheduler::check_tpl_task, celery::celery_init::init_impl};
use structopt::StructOpt;
use crate::rss::{ models::appenum::celery_opt::CeleryOpt};

pub mod rss;
pub mod model;

#[tokio::main]
async fn main() {
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
