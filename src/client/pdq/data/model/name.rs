use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Given {
    #[serde(rename = "$value")]
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Name {
    #[serde(rename = "@use")]
    pub use_: Option<String>,
    #[serde(rename = "given")]
    pub given: Given
}

impl Name {
    const COMPLETE_NAME_CODE: &'static str = "L";
    const SOCIAL_NAME_CODE: &'static str = "ASGN";

    pub fn is_social_name(&self) -> bool {
        return self.use_.as_ref().unwrap() == Self::SOCIAL_NAME_CODE;
    }

    pub fn is_complete_name(&self) -> bool {
        return self.use_.as_ref().unwrap() == Self::COMPLETE_NAME_CODE;
    }

    pub fn to_string(&self) -> String {
        return self.given.value.clone().unwrap_or_else(|| "".to_string());
    }
}