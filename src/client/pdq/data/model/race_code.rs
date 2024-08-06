use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RaceCode {
    #[serde(rename = "@code")]
    pub code: Option<String>,
}