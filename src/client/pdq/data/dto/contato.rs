use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContatoDTO {
    pub value: Option<String>,
    pub category: ContatoTipo,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ContatoTipo {
    TelefoneResidencial,
    TelefoneComercial,
    TelefoneContato,
    TelefoneOutros,
    Email,
    Celular,
    NaoDefinido,
}

impl ContatoTipo {
    pub fn from_str(value: &str) -> Self {
        match value {
            "PRN" => ContatoTipo::TelefoneResidencial,
            "WPN" => ContatoTipo::TelefoneComercial,
            "EMR" => ContatoTipo::TelefoneContato,
            "ORN" => ContatoTipo::TelefoneOutros,
            "NET" => ContatoTipo::Email,
            "ASN" => ContatoTipo::Celular,
            _ => ContatoTipo::NaoDefinido,
        }
    }
}