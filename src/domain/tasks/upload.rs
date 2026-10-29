use crate::domain::R2;
use crate::domain::task::{TaskContext, TaskStatus};
use crate::infrastructure::cloudflare::JsonFile;

use anyhow::Result;
use std::sync::OnceLock;
use tokio::runtime::Runtime;

static RUNTIME: OnceLock<Runtime> = OnceLock::new();

pub const RAW_DIR: &str = "chrematistics-raw/";

pub fn get_runtime() -> &'static Runtime {
    RUNTIME.get_or_init(|| Runtime::new().expect("failed to create tokio runtime"))
}

pub fn new_r2(ctx: &TaskContext, path: String, data: Vec<u8>) -> R2 {
    R2 {
        url: ctx.s3_url.clone(),
        access: ctx.s3.clone(),
        secret: ctx.s3_secret.clone(),
        bucket: "gear".into(),
        data,
        path,
    }
}

pub fn upload_to_cloudflare(ctx: &mut TaskContext) -> Result<TaskStatus> {
    if ctx.downloaded_file.is_none() {
        return Ok(TaskStatus::Skip("[None] downloaded_file".to_string()));
    }

    let path = format!("{}{}", RAW_DIR, ctx.downloaded_name);
    let ok = get_runtime().block_on(new_r2(ctx, path, ctx.downloaded_file.clone().unwrap()).upload());

    if !ok {
        return Ok(TaskStatus::Failed(format!(
            "upload {}",
            ctx.downloaded_name
        )));
    }

    Ok(TaskStatus::Success(ctx.s3_url.clone()))
}
