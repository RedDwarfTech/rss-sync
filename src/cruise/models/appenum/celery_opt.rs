use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "celery_app",
    about = "Run a Rust Celery producer or consumer.",
    setting = structopt::clap::AppSettings::ColoredHelp,
)]
pub enum CeleryOpt {
    Consume,
    Produce {
        #[structopt(possible_values = &["add", "buggy_task", "bound_task", "long_running_task"])]
        tasks: Vec<String>,
    },
}