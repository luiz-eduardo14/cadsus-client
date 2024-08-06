use serde::{Deserialize, Serialize};
use crate::client::pdq::data::model::name::Name;
use crate::client::pdq::data::model::relationship;

#[derive(Serialize, Deserialize, Debug)]
pub struct RelationshipCode {
    #[serde(rename = "@code")]
    pub code: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RelationshipHolder {
    pub name: Name,
}

pub const MOTHER_CODE: &str = "PRN";
pub const FATHER_CODE: &str = "NPRN";

#[derive(Serialize, Deserialize, Debug)]
pub enum RelationshipType {
    Mother,
    Father,
    None,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Relationship {
    #[serde(rename = "code")]
    pub relationship_code: RelationshipCode,
    #[serde(rename = "relationshipHolder1")]
    pub relationship_holder: RelationshipHolder,
}

impl Relationship {
    pub(crate) fn get_value(&self) -> (RelationshipType, String) {
        let code = self.relationship_code.code.clone();
        let relationship_type = match code.as_str() {
            relationship::MOTHER_CODE => RelationshipType::Mother,
            relationship::FATHER_CODE => RelationshipType::Father,
            _ => RelationshipType::None
        };

        let name = self.relationship_holder.name.given.value.clone().unwrap_or_else(|| "not found".to_string());
        return (relationship_type, name);
    }
}