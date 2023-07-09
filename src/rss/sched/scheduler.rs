use std::time::Duration;

use log::{info, error};
use tokio::time;

use crate::rss::{celery::celery_init::init_impl, models::appenum::celery_opt::CeleryOpt};

pub async fn check_tpl_task() {
    let mut interval = time::interval(Duration::from_millis(5000));
    loop {
        interval.tick().await;
        producer().await;
        info!("check tpl task");
        //consumer().await;
        //info!("check tpl task");
    }
}

async fn producer(){
    let result = init_impl(CeleryOpt::Produce { tasks: ["add".to_string()].to_vec() }).await;
    match result {
        Ok(_) => {
            info!("schedule producer task success")
        },
        Err(e) => {
            error!("schedule producer task failed, {}",e);
        },
    }
}

async fn consumer(){
    let result = init_impl(CeleryOpt::Consume).await;
    match result {
        Ok(_) => {
            info!("schedule task success")
        },
        Err(e) => {
            error!("schedule task failed, {}",e);
        }
    }
}