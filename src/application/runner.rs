use crate::domain::task::{Task, TaskContext, TaskStatus};
use anyhow::Result;
use std::thread;
use std::time::Duration;

pub fn run(mut tasks: Vec<Task>, context: &mut TaskContext) -> Result<()> {
    tasks.sort_by_key(|task| task.order);
    for task in tasks {
        println!("[Task] {} running: {}", task.order, task.name);

        match (task.func)(context) {
            Ok(TaskStatus::Success(reason)) => {
                println!("[Task] {} Succeed: {}", task.order, reason);
            }
            Ok(TaskStatus::Skip(reason)) => {
                println!("[Task] {} Skipped: {}", task.order, reason);
            }
            Ok(TaskStatus::Failed(reason)) => {
                println!("[Task] {} Failed: {}", task.order, reason);
            }
            Err(err) => {
                println!("[Task] {} Error: {:?}", task.order, err);
                break;
            }
        }

        if task.name.starts_with("download") {
            thread::sleep(Duration::from_secs(1));
            println!("[Task] {} sleep", task.order);
        }
    }
    Ok(())
}
