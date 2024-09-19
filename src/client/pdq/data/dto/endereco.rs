use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TipoEndereco {
    Casa,
    Trabalho,
    NaoInformado,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnderecoDTO {
    pub tipo: TipoEndereco,
    pub ibge: Option<String>,
    pub cep: Option<String>,
    pub bairro: Option<String>,
    pub numero_casa: Option<String>,
    pub nome_rua: Option<String>,
    pub codigo_tipo_rua: Option<String>,
    pub codigo_pais: Option<String>,
}