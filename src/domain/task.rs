use anyhow::Result;
use chrono::{DateTime, Local};

use crate::domain::tasks::{TFEX, TPEX, TWSE};
#[derive(Debug)]
pub enum TaskStatus {
    Skip(String),
    Failed(String),
    Success(String),
}

#[derive(Default, Clone)]
pub struct TaskContext {
    pub run_date: DateTime<Local>,
    pub downloaded_name: String,
    pub downloaded_file: Option<Vec<u8>>,
    /// migrate 用：(R2 key, gzip bytes)
    pub migrate_files: Vec<(String, Vec<u8>)>,
    pub url_twse: String,
    pub url_tpex: String,
    pub url_tfex: String,
    pub s3: String,
    pub s3_secret: String,
    pub s3_url: String,
}
pub struct Task {
    pub name: &'static str,
    pub order: u32,
    pub func: fn(&mut TaskContext) -> Result<TaskStatus>,
}

impl Task {
    pub fn new(
        order: u32,
        name: &'static str,
        func: fn(&mut TaskContext) -> Result<TaskStatus>,
    ) -> Self {
        Self { name, order, func }
    }
}
impl From<TaskContext> for TWSE {
    fn from(ctx: TaskContext) -> Self {
        TWSE {
            domain: ctx.url_twse,
        }
    }
}

impl From<TaskContext> for TPEX {
    fn from(ctx: TaskContext) -> Self {
        TPEX {
            domain: ctx.url_tpex,
        }
    }
}

impl From<TaskContext> for TFEX {
    fn from(ctx: TaskContext) -> Self {
        TFEX {
            domain: ctx.url_tfex,
        }
    }
}
