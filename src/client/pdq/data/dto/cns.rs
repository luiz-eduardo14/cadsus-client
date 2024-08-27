use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CNSSituation {
    DEFINITIVO,
    PROVISORIO,
    NONE,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CNSDTO {
    pub numero: Option<String>,
    pub situacao: Option<CNSSituation>,
}