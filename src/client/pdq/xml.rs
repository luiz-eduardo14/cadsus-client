use quick_xml::events::Event;

use crate::client::pdq::data::dto::cidadao::CidadaoDTO;
use crate::client::pdq::data::model::subject_root::SubjectRoot;

#[derive(Debug)]
pub enum XMLError {
    ConvertError(String),
    ReadXmlFailed(quick_xml::Error),
}

impl CidadaoDTO {
    /// Converts a XML reader into a vector of citizens
    pub fn vec_from_xml(mut reader: quick_xml::Reader<&[u8]>) -> Result<Vec<CidadaoDTO>, XMLError> {
        let mut citizens: Vec<CidadaoDTO> = Vec::new();
        loop {
            match reader.read_event() {
                Ok(Event::Start(ref e)) => {
                    if e.name().as_ref() == b"subject" {
                        let end = e.to_end().into_owned();
                        reader.config_mut().check_end_names = false;
                        match reader.read_text(end.name()) {
                            Ok(content) => {
                                let content = format!("<subject>{}</subject>", content);
                                let subject: Result<SubjectRoot, quick_xml::DeError> = quick_xml::de::from_str(&content);
                                if let Ok(subject) = subject {
                                    let result_citizen = subject.to_citizen_dto();
                                    if let Ok(citizen) = result_citizen {
                                        citizens.push(citizen);
                                    }
                                }
                            }
                            Err(e) => {
                                return Err(XMLError::ReadXmlFailed(e));
                            }
                        }
                        reader.config_mut().check_end_names = true;
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => Err(XMLError::ReadXmlFailed(e))?,
                _ => {}
            }
        }
        Ok(citizens)
    }
}