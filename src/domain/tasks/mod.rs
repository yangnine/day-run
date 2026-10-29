pub mod download;
pub mod market;
pub mod upload;
pub mod parser;
pub mod migrate;

#[derive(Debug, Clone, Copy)]
pub enum TWAction {
    /** MI_INDEX */
    Price,
    /** T86 量 三大法人*/
    Volumn,
    /** TWT38U 外資*/
    Foreign,
    /** TWT49U 除權息*/
    Dividend,
    /** BWIBBU_d 值利率*/
    InterestRate,
}

impl From<TWAction> for String {
    fn from(action: TWAction) -> Self {
        match action {
            TWAction::Price => "MI_INDEX".to_string(),
            TWAction::Volumn => "T86".to_string(),
            TWAction::InterestRate => "BWIBBU_d".to_string(),
            TWAction::Foreign => "TWT38U".to_string(),
            TWAction::Dividend => "TWT49U".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TPAction {
    /** otc */
    Price,
    /** dailyTrade */
    Volumn,
}

impl From<TPAction> for String {
    fn from(action: TPAction) -> Self {
        match action {
            TPAction::Price => "otc".to_string(),
            TPAction::Volumn => "dailyTrade".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TWSE {
    pub domain: String,
}

#[derive(Debug, Clone)]
pub struct TPEX {
    pub domain: String,
}

#[derive(Debug, Clone)]
pub struct TFEX {
    pub domain: String,
}
