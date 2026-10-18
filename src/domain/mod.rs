pub mod stock;
pub mod task;
pub mod tasks;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct R2 {
    pub url: String,
    pub access: String,
    pub secret: String,
    pub bucket: String,
    pub data: Vec<u8>,
    pub path: String
}
