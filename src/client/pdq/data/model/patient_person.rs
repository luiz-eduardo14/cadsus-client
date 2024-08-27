use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use crate::client::pdq::data::dto::cidadao::CidadaoDTO;
use crate::client::pdq::data::dto::cns::CNSDTO;
use crate::client::pdq::data::dto::contato::{ContatoDTO, ContatoTipo};
use crate::client::pdq::data::dto::nacionalidade::Nacionalidade;
use crate::client::pdq::data::dto::raca::Raca;
use crate::client::pdq::data::dto::sexo::Sexo;
use crate::client::pdq::data::model::address::Address;
use crate::client::pdq::data::model::administrative_gender_code::AdministrativeGenderCode;
use crate::client::pdq::data::model::birth_time::BirthTime;
use crate::client::pdq::data::model::birthPlace::BirthPlace;
use crate::client::pdq::data::model::deceased::Deceased;
use crate::client::pdq::data::model::id::{IdRoot, IdRootType};
use crate::client::pdq::data::model::name::Name;
use crate::client::pdq::data::model::race_code::RaceCode;
use crate::client::pdq::data::model::relationship::{Relationship, RelationshipType};
use crate::client::pdq::data::model::telecom::Telecom;

#[derive(Serialize, Deserialize, Debug)]
pub struct PatientPerson {
    #[serde(rename = "name")]
    pub names: Vec<Name>,
    #[serde(rename = "telecom")]
    pub telecoms: Vec<Telecom>,
    #[serde(rename = "administrativeGenderCode")]
    pub administrative_gender_code: AdministrativeGenderCode,
    #[serde(rename = "birthTime")]
    pub birth_time: BirthTime,
    #[serde(rename = "addr")]
    pub addresses: Vec<Address>,
    #[serde(rename = "raceCode")]
    pub race_code: RaceCode,
    #[serde(rename = "asOtherIDs")]
    pub other_ids: Vec<IdRoot>,
    #[serde(rename = "personalRelationship")]
    pub relationships: Vec<Relationship>,
    #[serde(rename = "deceasedInd")]
    pub deceased: Option<Deceased>,
    #[serde(rename = "deceasedTime")]
    pub deceased_time: Option<Deceased>,
    #[serde(rename = "birthPlace")]
    pub birth_place: Option<BirthPlace>,
}

impl PatientPerson {
    fn get_cns_and_cpf(&self) -> (Option<Vec<CNSDTO>>, Option<String>) {
        let mut cnss: Option<Vec<CNSDTO>> = None;
        let mut cpf: Option<String> = None;
        for id_root in self.other_ids.iter().map(|id_root| id_root.get_type()) {
            match id_root {
                IdRootType::CNS(cns) => {
                    if cnss.is_none() {
                        cnss = Some(vec![cns]);
                    } else {
                        cnss.as_mut().unwrap().push(cns);
                    }
                }
                IdRootType::CPF(cpf_value) => {
                    cpf = Some(cpf_value);
                }
                IdRootType::NotFound => {}
            }
        }
        return (cnss, cpf);
    }

    fn get_raca_cor(&self) -> Option<Raca> {
        return self.race_code.code.clone().map(|c| {
            return match c.as_str() {
                "01" => Raca::BRANCA,
                "02" => Raca::PRETA,
                "03" => Raca::PARDA,
                "04" => Raca::AMARELA,
                "05" => Raca::INDIGENA,
                _ => Raca::SEMINFORMACAO,
            };
        });
    }

    fn is_deceased(&self) -> bool {
        if let Some(deceased) = self.deceased.as_ref() {
            return deceased.value == "true";
        }
        if self.deceased_time.is_some() {
            return true;
        }

        return false;
    }

    fn get_deceased_date(&self) -> Option<NaiveDate> {
        if let Some(deceased) = self.deceased_time.as_ref() {
            let value = deceased.value.clone();
            return NaiveDate::parse_from_str(value.as_str(), "%Y%m%d%H%M%S").ok();
        }
        return None;
    }

    fn get_contatos(&self) -> Vec<ContatoDTO> {
        return self.telecoms.iter().map(|t| {
            let contact_type = ContatoTipo::from_str(t.use_.as_ref().unwrap().as_str());
            return ContatoDTO {
                value: t.value.clone(),
                category: contact_type,
            };
        }).collect();
    }

    pub(crate) fn to_dto(&self) -> CidadaoDTO {
        let mut citizen = CidadaoDTO::default();
        let names = self.names.clone();
        let (cnss, cpf) = self.get_cns_and_cpf();
        citizen.cnss = cnss;
        citizen.cpf = cpf;
        citizen.contatos = self.get_contatos();
        citizen.nome_completo = names.iter().find(|n| n.is_complete_name()).map(|n| n.to_string());
        citizen.nome_social = names.iter().find(|n| n.is_social_name()).map(|n| n.to_string());
        citizen.data_nascimento = self.birth_time.value.clone().map(|v| {
            return NaiveDate::parse_from_str(v.as_str(), "%Y%m%d%H%M%S").unwrap();
        });
        citizen.raca_cor = self.get_raca_cor();
        for relationship in self.relationships.iter() {
            let (relationship_type, name) = relationship.get_value();
            match relationship_type {
                RelationshipType::Mother => citizen.nome_mae = Some(name),
                RelationshipType::Father => citizen.nome_pai = Some(name),
                _ => {}
            }
        }
        citizen.vivo = !self.is_deceased();
        citizen.data_obito = self.get_deceased_date();
        citizen.enderecos = self.addresses.iter().filter_map(|a| a.to_dto()).collect();
        citizen.sexo = self.administrative_gender_code.code.clone().map(
            |c| match c.as_str() {
                "M" => Sexo::Masculino,
                "F" => Sexo::Feminino,
                _ => Sexo::Ignorado,
            }
        ).unwrap_or(Sexo::Ignorado);
        if let Some(birth_place) = self.birth_place.as_ref() {
            citizen.ibge_nascimento = birth_place.addr.as_ref()
                .and_then(|addr| addr.city.as_ref())
                .map(|c| c.value.clone());
            citizen.nacionalidade = birth_place.addr.as_ref().map(|c| {
                const BRASIL_CODE: &str = "010";
                if c.country.value == BRASIL_CODE {
                    return Nacionalidade::BRASILEIRO;
                }
                return Nacionalidade::ESTRANGEIRO;
            });
        }
        return citizen;
    }
}