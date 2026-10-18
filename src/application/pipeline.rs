use crate::{
    config::AppConfig, domain::{task::{Task, TaskContext}, tasks::{download, parser, upload::{self, upload_to_cloudflare}}},
};
use chrono::{DateTime, Datelike, Duration, Local, Weekday};

macro_rules! task {
    ($id:expr, $func:path) => {
        Task::new($id, stringify!($func), $func)
    };
}

/// 週末不開盤；國定假日無法用日期判斷，交給下載後的 stat 檢查
pub fn check_date_is_work_day(run_date: DateTime<Local>) -> bool {
    !matches!(run_date.weekday(), Weekday::Sat | Weekday::Sun)
}

pub fn build(config: AppConfig) -> (Vec<Task>, TaskContext) {
    // let run_date = Local::now() - Duration::days(1);
    let run_date = Local::now();
    let is_need_to_run = check_date_is_work_day(run_date); 
    // 股票每天晚上 5點半 有完整資料

    let context = TaskContext {
        url_twse: config.url_twse,
        url_tpex: config.url_tpex,
        url_tfex: config.url_tfex,
        s3: config.s3,
        s3_secret: config.s3_secret,
        s3_url: config.s3_url,
        run_date,
        ..Default::default()
    };
    if !is_need_to_run { return (vec![], context) }

    let tasks = vec![
        task!(0, download::twse_price),
        task!(0, parser::to_gz),
        task!(1, upload::upload_to_cloudflare),

        task!(2, download::twse_volume),
        task!(2, parser::to_gz),
        task!(3, upload::upload_to_cloudflare),

        task!(4, download::tpex_price),
        task!(4, parser::to_gz),
        task!(5, upload::upload_to_cloudflare),

        task!(6, download::tpex_volume),
        task!(6, parser::to_gz),
        task!(7, upload::upload_to_cloudflare),

        task!(8, download::tfex_volume),
        task!(8, parser::parse_csv),
        task!(8, parser::to_gz),
        task!(9, upload::upload_to_cloudflare),
    ];
    (tasks, context)
}
