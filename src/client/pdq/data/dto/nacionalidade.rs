use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Nacionalidade {
    Brasileiro,
    Estrangeiro,
    NaoInformado,
}