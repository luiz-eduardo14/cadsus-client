use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Sexo {
    Masculino,
    Feminino,
    Ignorado,
}