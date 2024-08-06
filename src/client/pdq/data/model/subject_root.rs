use serde::{Deserialize, Serialize};
use crate::client::pdq::data::dto::cidadao::CidadaoDTO;
use crate::client::pdq::data::model::registration_event::RegistrationEvent;
use crate::client::pdq::xml::XMLError;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "subject")]
pub struct SubjectRoot {
    #[serde(rename = "registrationEvent")]
    pub registration_event: RegistrationEvent,
}

impl SubjectRoot {
    pub fn to_citizen_dto(self) -> Result<CidadaoDTO, XMLError> {
        fn return_error_if_none<T>(value: Option<T>, error: String) -> Result<T, XMLError> {
            if value.is_none() {
                return Err(XMLError::ConvertError(error));
            }
            return Ok(value.unwrap());
        }
        let subject = return_error_if_none(self.registration_event.subject1, "Subject is None".to_string())?;
        let patient = return_error_if_none(subject.patient, "Patient is None".to_string())?;
        let patient_person = return_error_if_none(patient.patient_person, "PatientPerson is None".to_string())?;
        let citizen = patient_person.to_dto();
        return Ok(citizen);
    }
}