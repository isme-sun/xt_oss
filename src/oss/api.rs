use reqwest::Error as ReqwestError;
use serde::{Deserialize, Serialize};

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

// pub mod bucket;
// pub mod objects;
pub mod region;
pub mod service;

// match response {
//     Ok(response) => {
//         if response.status().is_success() {
//             Ok(Response(response))
//         } else {
//             let info = match response.headers().contains_key("x-oss-err") {
//                 true => {
//                     let info = response.headers().get("x-oss-err").unwrap();
//                     general_purpose::STANDARD.decode(info).unwrap().to_vec()
//                 }
//                 false => response.bytes().await.unwrap().to_vec(),
//             };
//             let content = String::from_utf8_lossy(&info);
//             let error: OssError = quick_xml::de::from_str(&content).unwrap();
//             Err(Error::OSS(error))
//         }
//     }
//     Err(error) => Err(Error::REQWEST(error)),
// }
