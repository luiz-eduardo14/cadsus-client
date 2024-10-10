use chrono::{NaiveDate, Utc};
use log::info;
use reqwest::Proxy;
use serde::{Deserialize, Serialize};
use tera::Tera;

use crate::client::pdq::data::dto::cidadao::CidadaoDTO;
use crate::client::pdq::xml::XMLError;

#[derive(Debug)]
pub struct Client {}

#[derive(Debug)]
pub enum CadsusRequestError {
    InvalidCns,
    InvalidCpf,
    InvalidDataNascimento,
    InvalidNome,
    InvalidNomeMae,
    UnspecifiedError(String),
    InternalServerError(String),
    Timeout,
    Unauthorized,
    FailedToRenderTemplate(String),
    XmlParse(XMLError),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct QueryParameters {
    pub cns: Option<String>,
    pub tenant: Option<String>,
    pub cpf: Option<String>,
    pub data_nascimento: Option<NaiveDate>,
    pub nome: Option<String>,
    pub nome_mae: Option<String>,
}

impl Default for QueryParameters {
    fn default() -> Self {
        QueryParameters {
            cns: None,
            tenant: None,
            cpf: None,
            data_nascimento: None,
            nome: None,
            nome_mae: None,
        }
    }
}

impl Client {
    const PDQ_URL: &'static str = "https://servicos.saude.gov.br/cadsus/v2/PDQSupplierJWT";

    fn get_context_from_query_parameters(
        parameters: &QueryParameters,
    ) -> tera::Context {
        let mut context = tera::Context::new();
        context.insert("cns", &parameters.cns);
        context.insert("tenant", &parameters.tenant);
        context.insert("cpf", &parameters.cpf);
        let data_nascimento = parameters.data_nascimento.map(|d| d.format("%Y%m%d").to_string());
        context.insert("data_nascimento", &data_nascimento);
        context.insert("nome", &parameters.nome);
        context.insert("nome_mae", &parameters.nome_mae);
        let now = Utc::now();
        context.insert("now", &now.format("%Y%m%d%H%M%S").to_string());
        context
    }

    fn generate_soap_cadsus_xml(
        parameters: &QueryParameters,
    ) -> Result<String, CadsusRequestError> {
        let mut tera = Tera::default();
        tera.add_raw_template("soap_cadsus.xml", include_str!("../../templates/PRPA_IN201305UV02.xml"))
            .unwrap();
        let context = Client::get_context_from_query_parameters(parameters);
        tera.render("soap_cadsus.xml", &context).map_err(|e| {
            CadsusRequestError::FailedToRenderTemplate(e.to_string())
        })
    }

    fn mount_cadsus_auth_header(
        obs_token: String,
    ) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/soap+xml".parse().unwrap());
        headers.insert("Authorization", format!("jwt {}", obs_token).parse().unwrap());
        headers
    }

    async fn fetch_cadsus_request(
        http_client: &reqwest::Client,
        headers: reqwest::header::HeaderMap,
        body: String,
    ) -> Result<reqwest::Response, CadsusRequestError> {
        let request = http_client.request(
            reqwest::Method::POST,
            Self::PDQ_URL,
        )
            .body(body)
            .headers(headers)
            .build()
            .map_err(|e| CadsusRequestError::UnspecifiedError(e.to_string()))?;
        let response = http_client.execute(request).await;
        return match response {
            Ok(response) => {
                Ok(response)
            }
            Err(e) => {
                if e.is_timeout() {
                    return Err(CadsusRequestError::Timeout);
                }

                Err(CadsusRequestError::UnspecifiedError(format!("Failed to execute request: {:?}", e)))
            }
        };
    }

    async fn process_cadsus_response(
        response: reqwest::Response,
    ) -> Result<Vec<CidadaoDTO>, CadsusRequestError> {
        let status = response.status();
        if status == reqwest::StatusCode::UNAUTHORIZED {
            return Err(CadsusRequestError::Unauthorized);
        }
        if status == reqwest::StatusCode::INTERNAL_SERVER_ERROR {
            if let Ok(body) = response.text().await {
                return Err(CadsusRequestError::InternalServerError(body));
            }
            return Err(CadsusRequestError::InternalServerError("
                Failed to get response body from server
            ".to_string()));
        }

        if status.is_success() {
            let body = response.text().await.map_err(|e| {
                CadsusRequestError::UnspecifiedError(e.to_string())
            })?;
            let reader = quick_xml::Reader::from_str(&body);
            return match CidadaoDTO::vec_from_xml(reader) {
                Ok(vec_cidadao) => Ok(vec_cidadao),
                Err(e) => Err(CadsusRequestError::XmlParse(e))
            };
        }

        Err(CadsusRequestError::UnspecifiedError(
            format!("Unexpected status code: {:?}", status)
        ))
    }

    pub async fn query_with_obs_token_and_proxy(
        parameters: &QueryParameters,
        obs_token: String,
        proxy: Proxy,
    ) -> Result<Vec<CidadaoDTO>, CadsusRequestError> {
        let client = reqwest::Client::builder()
            .use_rustls_tls()
            .tls_built_in_root_certs(true)
            .proxy(proxy)
            .build()
            .unwrap();

        let soap_xml = Self::generate_soap_cadsus_xml(parameters)?;
        let headers = Self::mount_cadsus_auth_header(obs_token);

        let response = Self::fetch_cadsus_request(&client, headers, soap_xml).await?;
        let citizen =  Self::process_cadsus_response(response).await?;
        Ok(citizen)
    }

    pub async fn query_with_obs_token(
        parameters: &QueryParameters,
        obs_token: String,
    ) -> Result<Vec<CidadaoDTO>, CadsusRequestError> {
        let client = reqwest::Client::builder()
            .use_rustls_tls()
            .tls_built_in_root_certs(true)
            .build()
            .unwrap();

        let soap_xml = Self::generate_soap_cadsus_xml(parameters)?;
        let headers = Self::mount_cadsus_auth_header(obs_token);

        let response = Self::fetch_cadsus_request(&client, headers, soap_xml).await?;
        let citizen =  Self::process_cadsus_response(response).await?;
        return Ok(citizen);
    }
}
