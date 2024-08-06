use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TituloEleitorDTO {
    pub numero: String,
    pub zona: String,
    pub secao: String
}