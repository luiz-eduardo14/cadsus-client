use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Raca {
    BRANCA,
    PRETA,
    PARDA,
    AMARELA,
    INDIGENA,
    SEMINFORMACAO
}