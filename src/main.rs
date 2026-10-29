use anyhow::Result;
use day_run::application::{pipeline, runner};
use day_run::config::AppConfig;

fn main() -> Result<()> {
    let conf = AppConfig::from_env();
    if conf.is_err() {
        println!("{:?}", conf);
        return Ok(());
    }

    let is_migrate = std::env::args().any(|arg| arg == "migrate");
    let (tasks, mut context) = if is_migrate {
        pipeline::build_migrate(conf?)
    } else {
        pipeline::build(conf?)
    };

    runner::run(tasks, &mut context)
}
