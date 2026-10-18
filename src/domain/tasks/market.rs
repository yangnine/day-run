use chrono::{DateTime, Local};

use crate::domain::tasks::{TFEX, TPAction, TPEX, TWAction, TWSE};

// stat	"OK"
// stat	"很抱歉，沒有符合條件的資料!"

//https://www.twse.com.tw/rwd/zh/afterTrading/MI_INDEX?date=20260904&type=ALLBUT0999&response=json&_=1788605736406
//https://www.twse.com.tw/rwd/zh/fund/T86?date=20260904&selectType=ALLBUT0999&response=json&_=1788605738603
impl TWSE {
    pub fn get_url(self, action: TWAction, date: DateTime<Local>) -> String {
        let timestamp = format!("&_={}", date.timestamp_millis());

        let (controller, action, datestr, select_type, response) = match action {
            TWAction::Price => (
                "/afterTrading",
                "/MI_INDEX",
                Self::get_url_date(date),
                "&type=ALLBUT0999",
                "&response=json",
            ),
            // 三大法人 量
            TWAction::Volumn => (
                "/fund",
                "/T86",
                Self::get_url_date(date),
                "&selectType=ALLBUT0999",
                "&response=json",
            ),
            // 值利率
            TWAction::InterestRate => (
                "/afterTrading",
                "/BWIBBU_d",
                Self::get_url_date(date),
                "&type=ALL",
                "",
            ),
            // 外資
            TWAction::Foreign => ("/fund", "/TWT38U", Self::get_url_date(date), "", ""),
            // 除權息
            TWAction::Dividend => (
                "/exRight",
                "/TWT49U",
                get_url_date_range(date),
                "&response=json",
                "",
            ),
        };

        format!(
            "{}{}{}{}{}{}{}",
            self.domain, controller, action, datestr, select_type, response, timestamp
        )
    }
    
    fn get_url_date(date: DateTime<Local>) -> String {
        format!("?date={}", date.naive_local().format("%Y%m%d").to_string())
    }
}

// stat	"ok"
// tables 0 data []
//https://www.tpex.org.tw/www/zh-tw/afterTrading/otc?date=2026/09/04&type=EW&response=json&_=1788605741078
//https://www.tpex.org.tw/www/zh-tw/insti/dailyTrade?date=2026/09/04&type=Daily&sect=EW&_=1788605743158
impl TPEX {
    pub fn get_url(self, action: TPAction, date: DateTime<Local>) -> String {
        let timestamp = format!("&_={}", date.timestamp_millis());
        let (controller, action, datestr, select_type, response) = match action {
            TPAction::Price => (
                "/afterTrading",
                "/otc",
                Self::get_url_date(date),
                "&type=EW",
                "&response=json",
            ),
            TPAction::Volumn => (
                "/insti",
                "/dailyTrade",
                Self::get_url_date(date),
                "&type=Daily&sect=EW",
                "",
            ),
        };

        format!(
            "{}{}{}{}{}{}{}",
            self.domain, controller, action, datestr, select_type, response, timestamp
        )
    }

    fn get_url_date(date: DateTime<Local>) -> String {
        format!(
            "?date={}",
            date.naive_local().format("%Y/%m/%d").to_string()
        )
    }
}

impl TFEX {
    pub fn get_url(self, date: DateTime<Local>) -> String {
        format!("{}/Dailydownload/DailydownloadCSV/{}.zip", self.domain, Self::get_url_date(date))
    }
    fn get_url_date(date: DateTime<Local>) -> String {
        let (yy, mm, dd) = get_yy_mm_dd(date);
        format!(
            "Daily_{}_{}_{}",yy,mm,dd
        )
    }
}

//https://mis.twse.com.tw/stock/index?lang=zhHant

//https://www.taifex.com.tw/cht/3/dlFutPrevious30DaysSalesData
//https://www.taifex.com.tw/file/taifex/Dailydownload/DailydownloadCSV/Daily_2026_07_28.zip

fn get_url_date_range(date: DateTime<Local>) -> String {
    let yy = date.naive_local().format("%Y").to_string();
    let year = yy.parse::<i32>().unwrap();
    format!("?startDate={}0101&endDate={}0101", year - 1, yy)
}

pub fn get_dir(date: DateTime<Local>) -> String {
    let (yy, mm, dd) = get_yy_mm_dd(date);
    format!("{}{}/{}", yy, mm, dd)
}
pub fn get_yy_mm_dd(date: DateTime<Local>) -> (String, String, String) {
    let naive = date.date_naive();
    let yy = naive.format("%Y").to_string();
    let mm = naive.format("%m").to_string();
    let dd = naive.format("%d").to_string();
    (yy, mm, dd)
}

pub fn check_market_open() -> bool {
    true
}
