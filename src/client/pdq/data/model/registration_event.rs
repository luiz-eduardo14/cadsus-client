use serde::{Deserialize, Serialize};
use crate::client::pdq::data::model::subject::Subject;

#[derive(Serialize, Deserialize, Debug)]
pub struct RegistrationEvent {
    #[serde(rename = "subject1")]
    pub subject1: Option<Subject>,
}