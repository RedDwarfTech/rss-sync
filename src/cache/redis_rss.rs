use rust_wheel::config::cache::redis_util::get_list_size;

pub fn get_task_count() -> usize {
    return get_list_size("celery").unwrap();
}
