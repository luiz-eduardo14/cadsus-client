use http::{Method, StatusCode};
use reqwest::{Client as HttpClient, Identity, Request, Url};
use serde::{Deserialize, Serialize};

use crate::client::osb::OsbTokenError::{InvalidPem, UnknownError};

const TOKEN_ENDPOINT: &str = "https://ehr-auth.saude.gov.br/api/osb/token";

#[derive(Serialize, Deserialize, Debug)]
pub struct OsbTokenResponse {
    pub access_token: String,
    pub scope: String,
    pub token_type: String,
    pub expires_in: i64,
}

#[derive(Debug)]
pub enum OsbTokenError {
    InvalidPem(String),
    TlsError(String),
    FailedRequest(String),
    UnknownError(String),
}

pub async fn get_osb_token_from_pem(pem_file: Vec<u8>) -> Result<OsbTokenResponse, OsbTokenError> {
    let identity = Identity::from_pem(&pem_file).map_err(
        |e| InvalidPem(format!("Falha ao criar a identidade: {:?}", e))
    )?;
    let client = HttpClient::builder()
        .use_rustls_tls()
        .tls_built_in_root_certs(true)
        .identity(identity)
        .https_only(true)
        .tls_info(true)
        .connection_verbose(true)
        .build()
        .map_err(
            |e| {
                OsbTokenError::TlsError(format!("Falha ao criar o cliente: {:?}", e))
            }
        )?;

    let request = Request::new(Method::GET, Url::parse(TOKEN_ENDPOINT).map_err(
        |e| {
            UnknownError(
                format!("Falha ao criar a url: {:?}", e)
            )
        }
    )?);
    let response = client.execute(request).await;

    return match response {
        Ok(response) => {
            let status = response.status();
            if status == StatusCode::OK {
                let body: OsbTokenResponse = response.json().await.map_err(
                    |e| {
                        OsbTokenError::FailedRequest(
                            format!("Falha ao deserializar o corpo da resposta: {:?}", e)
                        )
                    }
                )?;
                Ok(body)
            } else {
                Err(OsbTokenError::FailedRequest(format!("Falha na requisição: {:?}", status)))
            }
        }
        Err(e) => {
            return Err(OsbTokenError::FailedRequest(format!("Falha na requisição: {:?}", e)))
        }
    };
}
