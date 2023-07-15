use crate::{
    cache::redis_rss::get_task_count,
    cruise::{channel::rss_channel::fetch_channel_article, models::appenum::celery_opt::CeleryOpt},
    model::diesel::dolphin::custom_dolphin_models::RssSubSource,
    service::channel::channel_service::{
        get_channel_by_id, get_fresh_channel, update_pulled_channel,
    },
};
use celery::{prelude::TaskError, task::TaskResult};
use log::{error, info};

#[celery::task]
async fn add(x: i64, y: i32) -> TaskResult<i64> {
    info!("consumed message:{}{}", x, y);
    let success = handle_add(x).await;
    if success {
        Ok(x)
    } else {
        Err(TaskError::from(TaskError::UnexpectedError(
            "article pull error message".to_string(),
        )))
    }
}

async fn handle_add(channel_id: i64) -> bool {
    let channel = get_channel_by_id(channel_id);
    if channel.is_empty() {
        error!("get null channel,id:{}", channel_id);
        return false;
    }
    return fetch_channel_article(channel[0].clone()).await;
}

pub async fn init_impl(opt: &CeleryOpt) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let redis_addr =
        std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379/".into());
    let rss_app = celery::app!(
        broker = RedisBroker { redis_addr },
        tasks = [
            add
        ],
        task_routes = [
            "buggy_task" => "buggy-queue",
            "*" => "celery",
        ],
        prefetch_count = 1,
        heartbeat = Some(30)
    )
    .await?;

    match opt {
        CeleryOpt::Consume => {
            rss_app.display_pretty().await;
            rss_app.consume_from(&["celery", "buggy-queue"]).await?;
        }
        CeleryOpt::Produce { tasks } => {
            if tasks.is_empty() {
            } else {
                for task in tasks {
                    match task.as_str() {
                        "add" => {
                            if get_task_count() < 1 {
                                let refresh_rss: Vec<RssSubSource> = get_fresh_channel();
                                if !refresh_rss.is_empty() {
                                    let rss_record = refresh_rss[0].clone();
                                    rss_app.send_task(add::new(rss_record.id, 2)).await?;
                                    let _result = update_pulled_channel(rss_record);
                                }
                            }
                        }
                        _ => error!("unknown task"),
                    };
                }
            }
        }
    };

    rss_app.close().await?;
    Ok(())
}
