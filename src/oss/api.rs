use super::http::{HeaderMap, StatusCode, Url};
use base64::{engine::general_purpose, Engine as _};
use bytes::Bytes;
use reqwest::{Error as ReqwestError, Response};
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
pub enum Error {
    ReqwestError(ReqwestError),
    OssError(Message),
}

#[derive(Debug)]
pub struct Data<T> {
    url: Url,
    status: StatusCode,
    headers: HeaderMap,
    content: T,
}

impl<T> Data<T> {
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

type Result<T> = std::result::Result<Data<T>, Error>;

pub async fn into_api_result<T>(result: super::Result<Response>) -> Result<T>
where
    T: for<'a> Deserialize<'a>,
{
    use Error::{OssError, ReqwestError};
    match result {
        Ok(response) => {
            if response.status().is_success() {
                let url = response.url().to_owned();
                let status = response.status();
                let headers = response.headers().to_owned();
                let data: Bytes = response.bytes().await.unwrap().to_owned();
                let content = String::from_utf8_lossy(&data);
                let content: T = quick_xml::de::from_str(&content).unwrap();
                Ok(Data {
                    url,
                    status,
                    headers,
                    content,
                })
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
                Err(OssError(message))
            }
        }
        Err(error) => Err(ReqwestError(error)),
    }
}

pub(crate) mod region;
pub(crate) mod service;
