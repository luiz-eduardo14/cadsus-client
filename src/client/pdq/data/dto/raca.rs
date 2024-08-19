use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Raca {
    BRANCA,
    PRETA,
    PARDA,
    AMARELA,
    INDIGENA,
    SEMINFORMACAO
}