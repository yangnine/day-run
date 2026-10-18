use anyhow::{Context, Result};
use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub url_twse: String,
    pub url_tpex: String,
    pub url_tfex: String,
    pub s3: String,
    pub s3_secret: String,
    pub s3_url: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();
        Ok(Self {
            url_twse: required("URL_TWSE")?,
            url_tpex: required("URL_TPEX")?,
            url_tfex: required("URL_TFEX")?,
            s3: required("R2_ID")?,
            s3_secret: required("R2_SECRET")?,
            s3_url: required("R2_URL")
                .map(|id | format!("https://{}.r2.cloudflarestorage.com", id))?,
        })
    }
}

fn required(name: &str) -> Result<String> {
    env::var(name).with_context(|| format!("missing environment variable: {name}"))
}
// fn required_u32(name: &str) -> Result<u32> {
//     required(name)?
//         .parse()
//         .with_context(|| format!("{name} must be an integer"))
// }
