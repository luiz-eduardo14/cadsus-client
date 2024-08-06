use serde::{Deserialize, Serialize};

use crate::client::pdq::data::model::id::ID;
use crate::client::pdq::data::model::patient_person::PatientPerson;

#[derive(Serialize, Deserialize, Debug)]
pub struct Patient {
    #[serde(rename = "id")]
    pub ids: Option<Vec<ID>>,
    #[serde(rename = "patientPerson")]
    pub patient_person: Option<PatientPerson>,
}