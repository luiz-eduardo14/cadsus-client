use serde::{Deserialize, Serialize};
use crate::client::pdq::data::model::patient::Patient;
use crate::client::pdq::data::model::realm_code::RealmCode;

#[derive(Serialize, Deserialize, Debug)]
pub struct Subject {
    #[serde(rename = "realmCode")]
    pub realm_code: Option<RealmCode>,
    #[serde(rename = "patient")]
    pub patient: Option<Patient>
}
