use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Deceased {
    #[serde(rename = "@value")]
    pub value: String,
}