use std::time::Duration;

use log::{info, error};
use tokio::time;

use crate::rss::{celery::celery_init::init_impl, models::appenum::celery_opt::CeleryOpt};

pub async fn check_tpl_task(opt: &CeleryOpt) {
    let mut interval = time::interval(Duration::from_millis(5000));
    loop {
        interval.tick().await;
        producer(opt).await;
    }
}

async fn producer(opt: &CeleryOpt){
    let result = init_impl(opt).await;
    match result {
        Ok(_) => {
            info!("schedule producer task success")
        },
        Err(e) => {
            error!("schedule producer task failed, {}",e);
        },
    }
}