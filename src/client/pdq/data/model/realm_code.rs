use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RealmCode {
    #[serde(rename = "@code")]
    pub code: Option<String>,
}