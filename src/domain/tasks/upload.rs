use crate::domain::R2;
use crate::domain::task::{TaskContext, TaskStatus};
use crate::infrastructure::cloudflare::JsonFile;

use anyhow::Result;
use std::sync::OnceLock;
use tokio::runtime::Runtime;

static RUNTIME: OnceLock<Runtime> = OnceLock::new();

fn get_runtime() -> &'static Runtime {
    RUNTIME.get_or_init(|| Runtime::new().expect("failed to create tokio runtime"))
}

pub fn upload_to_cloudflare(ctx: &mut TaskContext) -> Result<TaskStatus> {
    if ctx.downloaded_file.is_none() {
        return Ok(TaskStatus::Skip("[None] downloaded_file".to_string()));
    }

    let ok = get_runtime().block_on(
        R2 {
            url: ctx.s3_url.clone(),
            access: ctx.s3.clone(),
            secret: ctx.s3_secret.clone(),
            bucket: "gear".into(),
            data: ctx.downloaded_file.clone().unwrap(),
            path: format!("chrematistics-raw/{}", ctx.downloaded_name),
        }
        .upload(),
    );

    if !ok {
        return Ok(TaskStatus::Failed(format!(
            "upload {}",
            ctx.downloaded_name
        )));
    }

    Ok(TaskStatus::Success(ctx.s3_url.clone()))
}

pub fn save_local_test(ctx: &mut TaskContext) -> Result<TaskStatus> {
    if ctx.downloaded_file.is_none() {
        return Ok(TaskStatus::Skip("[None] downloaded_file".to_string()));
    }

    let local_path = format!("./tests/data/{}", ctx.downloaded_name);

    if let Some(parent) = std::path::Path::new(&local_path).parent() {
        std::fs::create_dir_all(parent)?;
    }

    std::fs::write(&local_path, ctx.downloaded_file.clone().unwrap())?;

    Ok(TaskStatus::Success(ctx.downloaded_name.clone()))
}
