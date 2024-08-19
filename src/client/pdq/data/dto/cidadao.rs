use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::client::pdq::data::dto::certidao::CertidaoDTO;
use crate::client::pdq::data::dto::cns::CNSDTO;
use crate::client::pdq::data::dto::contato::ContatoDTO;
use crate::client::pdq::data::dto::endereco::EnderecoDTO;
use crate::client::pdq::data::dto::raca::Raca;
use crate::client::pdq::data::dto::sexo::Sexo;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CidadaoDTO {
    pub cnss: Option<Vec<CNSDTO>>,
    pub cpf: Option<String>,
    pub contatos: Vec<ContatoDTO>,
    pub certidoes: Vec<CertidaoDTO>,
    pub enderecos: Vec<EnderecoDTO>,
    pub data_nascimento: Option<NaiveDate>,
    pub data_obito: Option<NaiveDate>,
    pub nome_completo: Option<String>,
    pub nome_mae: Option<String>,
    pub nome_pai: Option<String>,
    pub nome_social: Option<String>,
    pub raca_cor: Option<Raca>,
    pub sexo: Sexo,
    pub vivo: bool,
}

impl Default for CidadaoDTO {
    fn default() -> Self {
        CidadaoDTO {
            cnss: None,
            contatos: vec![],
            certidoes: vec![],
            enderecos: vec![],
            cpf: None,
            data_nascimento: None,
            data_obito: None,
            nome_completo: None,
            nome_mae: None,
            nome_pai: None,
            nome_social: None,
            raca_cor: None,
            sexo: Sexo::Ignorado,
            vivo: false,
        }
    }
}