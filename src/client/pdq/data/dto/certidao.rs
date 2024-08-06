use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CertidaoDTO {
    pub cartorio: Option<String>,
    pub folha: Option<String>,
    pub livro: Option<String>,
    pub termo: Option<String>,
    pub data_emissao: Option<String>,
    pub matricula: Option<String>,
    pub tipo: Option<String>,
}
