use serde::{Deserialize, Serialize};
use crate::client::pdq::data::dto::endereco::{EnderecoDTO, TipoEndereco};

#[derive(Serialize, Deserialize, Debug)]
pub struct City {
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostalCode {
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Country {
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HouseNumber {
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StreetName {
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StreetNameType {
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AdditionalLocator {
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Address {
    #[serde(rename = "@use")]
    pub use_: Option<String>,
    #[serde(rename = "city")]
    pub city: Option<City>,
    #[serde(rename = "postalCode")]
    pub postal_code: Option<PostalCode>,
    #[serde(rename = "country")]
    pub country: Option<Country>,
    #[serde(rename = "houseNumber")]
    pub house_number: Option<HouseNumber>,
    #[serde(rename = "streetName")]
    pub street_name: Option<StreetName>,
    #[serde(rename = "streetNameType")]
    pub street_name_type: Option<StreetNameType>,
    #[serde(rename = "additionalLocator")]
    pub additional_locator: Option<AdditionalLocator>,
}

impl Address {
    pub fn to_dto(&self) -> Option<EnderecoDTO> {
        let city = self.city.as_ref().map(|c| c.value.clone());
        let postal_code = self.postal_code.as_ref().map(|p| p.value.clone());
        let country = self.country.as_ref().map(|country| country.value.clone());
        let house_number = self.house_number.as_ref().map(|h| h.value.clone());
        let street_name = self.street_name.as_ref().map(|s| s.value.clone());
        let street_name_type = self.street_name_type.as_ref().map(|s| s.value.clone());
        let address_type = self.use_.clone().map(
            |t| match t.as_str() {
                "H" => TipoEndereco::Casa,
                "WP" => TipoEndereco::Trabalho,
                _ => TipoEndereco::NaoInformado,
            }
        ).unwrap_or(TipoEndereco::NaoInformado);
        return Some(EnderecoDTO {
            tipo: address_type,
            ibge: city,
            cep: postal_code,
            bairro: self.additional_locator.as_ref().map(|a| a.value.clone()),
            numero_casa: house_number,
            nome_rua: street_name,
            codigo_tipo_rua: street_name_type,
            codigo_pais: country,
        });
    }
}