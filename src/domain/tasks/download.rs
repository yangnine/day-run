use crate::domain::task::{TaskContext, TaskStatus};
use crate::domain::tasks::market::get_dir;
use crate::domain::tasks::{TFEX, TPAction, TPEX, TWAction, TWSE};

use anyhow::Result;
use std::io::{Cursor, Read};
use zip::ZipArchive;
use bytes::Bytes;

pub fn twse_price(ctx: &mut TaskContext) -> Result<TaskStatus> {
    download_from(ctx, ExchangeKind::Twse, DataKind::Price)
}

pub fn twse_volume(ctx: &mut TaskContext) -> Result<TaskStatus> {
    download_from(ctx, ExchangeKind::Twse, DataKind::Volume)
}

pub fn tpex_price(ctx: &mut TaskContext) -> Result<TaskStatus> {
    download_from(ctx, ExchangeKind::Tpex, DataKind::Price)
}

pub fn tpex_volume(ctx: &mut TaskContext) -> Result<TaskStatus> {
    download_from(ctx, ExchangeKind::Tpex, DataKind::Volume)
}

pub fn tfex_volume(ctx: &mut TaskContext) -> Result<TaskStatus> {
    download_from(ctx, ExchangeKind::Taifex, DataKind::Price)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExchangeKind {
    Twse,
    Tpex,
    Taifex,
}

#[derive(Debug, Clone, Copy)]
pub enum DataKind {
    Price,
    Volume,
}

pub fn download_from(
    ctx: &mut TaskContext,
    exchange: ExchangeKind,
    kind: DataKind,
) -> Result<TaskStatus> {
    ctx.downloaded_file = None;
    ctx.downloaded_name = "".to_string();

    let url = match exchange {
        ExchangeKind::Twse => {
            let twse: TWSE = ctx.clone().into();
            let action = match kind {
                DataKind::Price => TWAction::Price,
                DataKind::Volume => TWAction::Volumn,
            };
            let name: String = action.into();
            ctx.downloaded_name = format!("{}_{}.json", get_dir(ctx.run_date), name);
            twse.get_url(action, ctx.run_date)
        }
        ExchangeKind::Tpex => {
            let tpex: TPEX = ctx.clone().into();
            let action = match kind {
                DataKind::Price => TPAction::Price,
                DataKind::Volume => TPAction::Volumn,
            };
            let name: String = action.into();
            ctx.downloaded_name = format!("{}_{}.json", get_dir(ctx.run_date), name);
            tpex.get_url(action, ctx.run_date)
        }
        ExchangeKind::Taifex => {
            let tfex: TFEX = ctx.clone().into();
            ctx.downloaded_name = format!("{}_TFex.json", get_dir(ctx.run_date));
            tfex.get_url(ctx.run_date)
        }
    };

    let res = reqwest::blocking::get(url.clone());

    match res {
        Err(_) => Ok(TaskStatus::Failed(format!("{}: {}", url, "沒有回應"))),
        Ok(response) => {
            if !response.status().is_success() {
                return Ok(TaskStatus::Failed(format!("{:?}", response.status())));
            }

            let byte = match response.bytes() {
                Ok(b) => b,
                Err(e) => return Ok(TaskStatus::Failed(format!("{}: 讀取內容失敗 {}", url, e))),
            };

            ctx.downloaded_file = if exchange == ExchangeKind::Taifex {
                unzip(byte)
            } else {
                Some(byte.to_vec())
            };
            Ok(TaskStatus::Success(url))
        }
    }
}

fn unzip(byte: Bytes) -> Option<Vec<u8>> {
    let mut archive = ZipArchive::new(Cursor::new(byte)).ok()?;

    let mut file = archive.by_index(0).ok()?;

    let mut contents = Vec::new();
    file.read_to_end(&mut contents).ok()?;

    Some(contents)
}
