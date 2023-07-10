use celery::task::TaskResult;
use log::info;

use crate::cruise::{models::appenum::celery_opt::CeleryOpt, channel::rss_channel::fetch_channel_article};

#[celery::task]
async fn add(x: i32, y: i32) -> TaskResult<i32> {
    info!("consumed message:{}{}", x, y);
    fetch_channel_article().await;
    Ok(x + y)
}

pub async fn init_impl(opt: &CeleryOpt) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let redis_addr =
        std::env::var("REDIS_ADDR").unwrap_or_else(|_| "redis://127.0.0.1:6379/".into());
    info!("redis addr:{}", redis_addr);
    let my_app = celery::app!(
        broker = RedisBroker { redis_addr },
        tasks = [
            add
        ],
        task_routes = [
            "buggy_task" => "buggy-queue",
            "*" => "celery",
        ],
        prefetch_count = 2,
        heartbeat = Some(10)
    )
    .await?;

    match opt {
        CeleryOpt::Consume => {
            my_app.display_pretty().await;
            my_app.consume_from(&["celery", "buggy-queue"]).await?;
        }
        CeleryOpt::Produce { tasks } => {
            if tasks.is_empty() {
                my_app.send_task(add::new(1, 2)).await?;
            } else {
                for task in tasks {
                    match task.as_str() {
                        "add" => my_app.send_task(add::new(1, 2)).await?,
                        _ => panic!("unknown task"),
                    };
                }
            }
        }
    };

    my_app.close().await?;
    Ok(())
}
