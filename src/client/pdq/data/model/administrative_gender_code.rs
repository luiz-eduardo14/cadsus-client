use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AdministrativeGenderCode {
    #[serde(rename = "@code")]
    pub code: Option<String>,
    #[serde(rename = "@codeSystem")]
    pub code_system: Option<String>,
}