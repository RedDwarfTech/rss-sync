#![allow(non_upper_case_globals)]
extern crate openssl;

use log::{info, error};
use std::time::Duration;
use tokio::{select, time};

use crate::rss::{celery::celery_init::init_impl, models::appenum::celery_opt::CeleryOpt};

pub mod rss;

#[tokio::main]
async fn main() {
    let mut interval1 = time::interval(Duration::from_secs(15));
    let mut interval2 = time::interval(Duration::from_secs(30));

    loop {
        select! {
                    _ = interval1.tick() => {
                        // 执行任务1
                        let result = init_impl(CeleryOpt::Consume).await;
                        match result {
                            Ok(_) => {
                                info!("schedule task success")
                            },
                            Err(e) => {
                                error!("schedule task failed, {}",e);
                            },
        }
                    }
                    _ = interval2.tick() => {
                        // 执行任务2
                    }
                }
    }
}
