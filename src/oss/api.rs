use super::{
    http::{self, HeaderMap, StatusCode, Url},
    Bytes, Response,
};
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ErrorMessage {
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

impl fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]: {}", self.code, self.message)
    }
}

#[derive(Debug)]
// api 返回数据， 可能正确也可能错误
pub struct ApiData<T> {
    pub(crate) url: Url,
    pub(crate) status: StatusCode,
    pub(crate) headers: HeaderMap,
    pub(crate) content: T,
}

impl<T> ApiData<T> {
    pub fn url(&self) -> &Url {
        &self.url
    }

    pub fn status(&self) -> &StatusCode {
        &self.status
    }

    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    pub fn content(self) -> T {
        self.content
    }

    pub fn request_id(&self) -> String {
        self.headers.get("x-oss-request-id").unwrap().to_str().unwrap().into()
    }
}

pub type ApiResponse<T> = Result<ApiData<T>, ApiData<ErrorMessage>>;

// api 返回体， 包含请求错误， 和api返回数据
pub type ApiResult<T = ()> = Result<ApiResponse<T>, reqwest::Error>;

pub(crate) struct ApiResponseFrom(reqwest::Response);

impl ApiResponseFrom {
    pub(crate) async fn fail_message(resp: Response) -> ApiData<ErrorMessage> {
        assert!(!resp.status().is_success());
        let url = resp.url().clone();
        let status = resp.status().clone();
        let headers = resp.headers().clone();
        let info = match resp.headers().contains_key("x-oss-err") {
            true => {
                let info = resp.headers().get("x-oss-err").unwrap();
                general_purpose::STANDARD.decode(info).unwrap().to_vec()
            }
            false => resp.bytes().await.unwrap().to_vec(),
        };
        let content = String::from_utf8_lossy(&info);
        let content: ErrorMessage = quick_xml::de::from_str(&content).unwrap();
        ApiData {
            url,
            status,
            headers,
            content,
        }
    }

    pub(crate) async fn bytes_data(resp: Response) -> ApiData<Bytes> {
        assert!(resp.status().is_success());
        let url = resp.url().clone();
        let status = resp.status().clone();
        let headers = resp.headers().clone();
        let content = resp.bytes().await.unwrap();
        ApiData {
            url,
            status,
            headers,
            content,
        }
    }

    pub(crate) async fn to_type<T>(self) -> ApiResponse<T>
    where
        T: for<'a> Deserialize<'a>,
    {
        let resp = self.0;
        if resp.status().is_success() {
            let url = resp.url().clone();
            let status = resp.status().clone();
            let headers = resp.headers().clone();
            let content = resp.bytes().await.unwrap();
            let content = String::from_utf8_lossy(&content);
            let content: T = quick_xml::de::from_str(&content).unwrap();

            Ok(ApiData {
                url,
                status,
                headers,
                content,
            })
        } else {
            let data_fail_message = Self::fail_message(resp).await;
            Err(data_fail_message)
        }
    }

    pub(crate) async fn to_bytes(self) -> ApiResponse<Bytes> {
        let resp = self.0;
        if resp.status().is_success() {
            let data = Self::bytes_data(resp).await;
            Ok(data)
        } else {
            let data_fail_message = Self::fail_message(resp).await;
            Err(data_fail_message)
        }
    }

    pub(crate) async fn to_text(self) -> ApiResponse<String> {
        let resp = self.0;
        if resp.status().is_success() {
            let url = resp.url().clone();
            let status = resp.status().clone();
            let headers = resp.headers().clone();
            let content = resp.text().await.unwrap();
            Ok(ApiData {
                url,
                status,
                headers,
                content,
            })
        } else {
            let data_fail_message = Self::fail_message(resp).await;
            Err(data_fail_message)
        }
    }

    pub(crate) async fn to_empty(self) -> ApiResponse<()> {
        let resp = self.0;
        if resp.status().is_success() {
            Ok(ApiData {
                url: resp.url().clone(),
                status: resp.status().clone(),
                headers: resp.headers().clone(),
                content: (),
            })
        } else {
            let data_fail_message = Self::fail_message(resp).await;
            Err(data_fail_message)
        }
    }
}



fn insert_header<T: ToString + std::fmt::Display>(
    headers: &mut http::HeaderMap,
    key: http::header::HeaderName,
    value: T,
) {
    headers.insert(key, value.to_string().parse().expect("Failed to parse header value"));
}

fn insert_custom_header<T: ToString + std::fmt::Display>(headers: &mut http::HeaderMap, key: &str, value: T) {
    let header_name = http::HeaderName::from_bytes(key.as_bytes()).expect("Failed to create header name");
    headers.insert(
        header_name,
        value.to_string().parse().expect("Failed to parse header value"),
    );
}


pub(crate) mod bucket;
pub(crate) mod objects;
pub(crate) mod region;
pub(crate) mod service;
