use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BirthTime {
    #[serde(rename = "@value")]
    pub value: Option<String>,
}