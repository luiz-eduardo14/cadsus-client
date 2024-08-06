use serde::{Deserialize, Serialize};
use crate::client::pdq::data::dto::cns::{CNSDTO, Situations};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum IdType {
    CnsNumber,
    CnsSituation,
    Cpf,
    NotFound,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ID {
    #[serde(rename = "@root")]
    pub root: Option<String>,
    #[serde(rename = "@extension")]
    pub extension: Option<String>,
    #[serde(rename = "@assigningAuthorityName")]
    pub assigning_authority_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IdRoot {
    #[serde(rename = "@classCode")]
    pub class_code: String,
    #[serde(rename = "id")]
    pub ids: Vec<ID>,
}

pub enum IdRootType {
    CNS(CNSDTO),
    CPF(String),
    NotFound,
}

impl IdRoot {
    pub(crate) fn get_type(&self) -> IdRootType {
        let first_id = self.ids.first();
        if first_id.is_none() {
            return IdRootType::NotFound;
        }
        let id = first_id.unwrap();
        match id.get_type() {
            IdType::CnsNumber | IdType::CnsSituation => {
                let mut number: Option<String> = None;
                let mut situation: Option<String> = None;
                for id in self.ids.iter() {
                    match id.get_type() {
                        IdType::CnsNumber => {
                            number = id.extension.clone();
                        }
                        IdType::CnsSituation => {
                            situation = id.extension.clone();
                        }
                        _ => {}
                    }
                }
                return IdRootType::CNS(CNSDTO {
                    numero: number,
                    situacao: situation.map(|s| {
                        return match s.as_str() {
                            "D" => Situations::DEFINITIVO,
                            "P" => Situations::PROVISORIO,
                            _ => Situations::NONE,
                        };
                    }),
                });
            }
            IdType::Cpf => IdRootType::CPF(id.extension.clone().unwrap()),
            _ => IdRootType::NotFound
        }
    }
}

impl ID {
    const CNS_NUMBER: &'static str = "2.16.840.1.113883.13.236";
    const CNS_SITUATION: &'static str = "2.16.840.1.113883.13.236.1";
    const CPF: &'static str = "2.16.840.1.113883.13.237";

    pub fn get_type(&self) -> IdType {
        return match self.root.as_ref().unwrap().as_str() {
            Self::CNS_NUMBER => IdType::CnsNumber,
            Self::CNS_SITUATION => IdType::CnsSituation,
            Self::CPF => IdType::Cpf,
            _ => IdType::NotFound
        };
    }
}

