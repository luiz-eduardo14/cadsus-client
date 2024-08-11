use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Sexo {
    Masculino,
    Feminino,
    Ignorado,
}