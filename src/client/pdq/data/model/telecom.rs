use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Telecom {
    #[serde(rename = "@value")]
    pub value: Option<String>,
    #[serde(rename = "@use")]
    pub use_: Option<String>,
}