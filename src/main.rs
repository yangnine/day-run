use anyhow::Result;
use day_run::application::{pipeline, runner};
use day_run::config::AppConfig;

fn main() -> Result<()> {
    let conf = AppConfig::from_env();
    if conf.is_err() {
        println!("{:?}", conf);
        return Ok(());
    }

    let (tasks, mut context) = pipeline::build(conf?);

    runner::run(tasks, &mut context)
}
