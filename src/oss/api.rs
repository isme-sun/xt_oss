use super::{
    http::{HeaderMap, StatusCode, Url},
    Bytes, Result,
};
use base64::{engine::general_purpose, Engine as _};
use reqwest::{Error, Response};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Message {
    #[serde(rename(deserialize = "Code"))]
    pub code: String,
    #[serde(rename(deserialize = "Message"))]
    pub message: String,
    #[serde(rename(deserialize = "RequestId"))]
    pub request_id: String,
    #[serde(rename(deserialize = "HostId"))]
    pub host_id: String,
    #[serde(rename(deserialize = "EC"))]
    pub ec: Option<String>,
    #[serde(rename(deserialize = "RecommendDoc"))]
    pub recommend_doc: Option<String>,
    #[serde(rename = "OSSAccessKeyId")]
    pub oss_access_key_id: Option<String>,
    #[serde(rename = "SignatureProvided")]
    pub signature_provided: Option<String>,
    #[serde(rename = "StringToSign")]
    pub string_to_sign: Option<String>,
    #[serde(rename = "StringToSignBytes")]
    pub string_to_sign_bytes: Option<String>,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]: {}", self.code, self.message)
    }
}

#[derive(Debug)]
pub struct Data<T> {
    url: Url,
    status: StatusCode,
    headers: HeaderMap,
    content: T,
}

impl<T> Data<T> {
    pub fn request_id(&self) -> String {
        "".to_string()
    }

    pub fn url(&self) -> &Url {
        &self.url
    }

    pub fn status(&self) -> &StatusCode {
        &self.status
    }

    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    pub fn content(&self) -> &T {
        &self.content
    }
}

pub enum ApiResponse<T> {
    SUCCESS(Data<T>),
    FAIL(Data<Message>),
}

#[allow(unused)]
type ApiResult<T> = std::result::Result<ApiResponse<T>, Error>;

pub(crate) async fn into_api_result(result: Result<Response>) -> ApiResult<Bytes> {
    use ApiResponse::{FAIL, SUCCESS};
    match result {
        Ok(response) => {
            let url = response.url().to_owned();
            let status = response.status();
            let headers = response.headers().to_owned();
            if response.status().is_success() {
                let content: Bytes = response.bytes().await.unwrap().to_owned();
                Ok(SUCCESS(Data {
                    url,
                    status,
                    headers,
                    content,
                }))
            } else {
                let info = match response.headers().contains_key("x-oss-err") {
                    true => {
                        let info = response.headers().get("x-oss-err").unwrap();
                        general_purpose::STANDARD.decode(info).unwrap().to_vec()
                    }
                    false => response.bytes().await.unwrap().to_vec(),
                };
                let content = String::from_utf8_lossy(&info);
                let message: Message = quick_xml::de::from_str(&content).unwrap();
                Ok(FAIL(Data {
                    url,
                    status,
                    headers,
                    content: message,
                }))
            }
        }
        Err(error) => Err(error),
    }
}

pub(crate) mod bucket;
pub(crate) mod region;
pub(crate) mod service;
