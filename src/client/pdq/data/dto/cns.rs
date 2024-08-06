use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Situations {
    DEFINITIVO,
    PROVISORIO,
    NONE,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CNSDTO {
    pub numero: Option<String>,
    pub situacao: Option<Situations>,
}