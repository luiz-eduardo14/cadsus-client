use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Situations {
    DEFINITIVO,
    PROVISORIO,
    NONE,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CNSDTO {
    pub numero: Option<String>,
    pub situacao: Option<Situations>,
}