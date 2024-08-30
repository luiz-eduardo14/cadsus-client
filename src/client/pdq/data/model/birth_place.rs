use serde::{Deserialize, Serialize};
use crate::client::pdq::data::model::address::Address;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Given {
    #[serde(rename = "$value")]
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BirthPlace {
    #[serde(rename = "addr")]
    pub addr: Option<Address>,
}