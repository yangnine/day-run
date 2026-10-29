//! 一次性：把 R2 上舊的 .gz 檔重新上傳，套用新的 header
//! (content-type: application/json + content-encoding: gzip)
use crate::domain::task::{TaskContext, TaskStatus};
use crate::domain::tasks::upload::{RAW_DIR, get_runtime, new_r2};
use crate::infrastructure::cloudflare::JsonFile;

use anyhow::Result;

pub fn download_gz_from_cloudflare(ctx: &mut TaskContext) -> Result<TaskStatus> {
    ctx.migrate_files.clear();

    let rt = get_runtime();
    let keys = rt.block_on(new_r2(ctx, RAW_DIR.into(), vec![]).list());
    if keys.is_empty() { return Ok(TaskStatus::Skip(format!("no .gz under {RAW_DIR}"))) }

    for key in keys {
        match rt.block_on(new_r2(ctx, key.clone(), vec![]).download()) {
            Some(data) => ctx.migrate_files.push((key, data)),
            None => println!("[migrate] download failed: {key}"),
        }
    }
    Ok(TaskStatus::Success(format!("{} files", ctx.migrate_files.len())))
}

/// 上傳前先備份到本機，上傳失敗還能還原
pub fn save_migrate_files_local(ctx: &mut TaskContext) -> Result<TaskStatus> {
    if ctx.migrate_files.is_empty() { return Ok(TaskStatus::Skip("no files to save".into())) }

    for (key, data) in &ctx.migrate_files {
        let local_path = format!("./test/data/{key}");
        if let Some(parent) = std::path::Path::new(&local_path).parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&local_path, data)?;
    }
    Ok(TaskStatus::Success(format!("{} files saved", ctx.migrate_files.len())))
}

pub fn reupload_with_json_header(ctx: &mut TaskContext) -> Result<TaskStatus> {
    if ctx.migrate_files.is_empty() { return Ok(TaskStatus::Skip("no files to upload".into())) }

    let rt = get_runtime();
    let mut failed = 0;
    for (key, data) in &ctx.migrate_files {
        let ok = rt.block_on(new_r2(ctx, key.clone(), data.clone()).upload());
        if !ok {
            failed += 1;
            println!("[migrate] upload failed: {key}");
        }
    }

    let summary = format!("{} ok, {} failed", ctx.migrate_files.len() - failed, failed);
    if failed > 0 { return Ok(TaskStatus::Failed(summary)) }
    Ok(TaskStatus::Success(summary))
}
