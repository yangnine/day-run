use crate::domain::task::{TaskContext, TaskStatus};

use std::collections::HashMap;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use flate2::Compression;
use flate2::write::GzEncoder;
use std::io::Write;

#[derive(Debug, Serialize, Deserialize)]
pub struct TpexIndexResponse {
    pub stat: String,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaifexRecord {
    #[serde(rename = "成交日期")]
    trade_date: String,
    #[serde(rename = "商品代號")]
    product_id: String,
    #[serde(rename = "到期月份(週別)")]
    contract_month: String,
    #[serde(rename = "成交時間")]
    trade_time: String,
    #[serde(rename = "成交價格")]
    price: String, // 有些欄位可能是 "-",不能直接用 f64
    #[serde(rename = "成交數量(B+S)")]
    volume: String,
    #[serde(rename = "近月價格")]
    near_month_price: String,
    #[serde(rename = "遠月價格")]
    far_month_price: String,
    #[serde(rename = "開盤集合競價")]
    opening_auction: String,
}

pub fn parse_json(ctx: &mut TaskContext) -> Result<TaskStatus> {
    let res = serde_json::from_slice::<TpexIndexResponse>(&ctx.downloaded_file.clone().unwrap());
    let data = match res {
        Ok(data) => data,
        Err(e) => {
            eprintln!("parse json error: {:?}", e);
            return Ok(TaskStatus::Failed("parser json error".to_string()));
        }
    };
    Ok(TaskStatus::Success("parser ok".to_string()))
}

pub fn parse_csv(ctx: &mut TaskContext) -> Result<TaskStatus> {
    let bytes = ctx.downloaded_file.as_ref().unwrap();
    let mut rdr = csv::Reader::from_reader(bytes.as_slice());
    let mut records = Vec::new();
    for result in rdr.deserialize() {
        let record: TaifexRecord = result?;
        records.push(record);
    }

    let records: Vec<TaifexRecord> = records
        .into_iter()
        .filter(|f| f.product_id.trim() == "MTX")
        .collect();

    let mut wtr = csv::Writer::from_writer(Vec::new());
    for record in &records {
        wtr.serialize(record)?;
    }
    let bytes = wtr.into_inner()?; // Vec<u8>

    ctx.downloaded_file = Some(bytes);

    Ok(TaskStatus::Success("parser ok".to_string()))
}

pub fn to_gz(ctx: &mut TaskContext) -> Result<TaskStatus> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
    encoder.write_all(&ctx.downloaded_file.clone().unwrap())?;
    let compressed = encoder.finish()?;

    ctx.downloaded_file = Some(compressed);
    ctx.downloaded_name = format!("{}.gz", ctx.downloaded_name);

    // std::fs::write(format!("./tests/data/{}.gz", ctx.downloaded_name), compressed)?;
    Ok(TaskStatus::Success(ctx.downloaded_name.clone()))
}
