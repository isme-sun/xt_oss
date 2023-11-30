// use reqwest::Client;
// use xt_oss::{DEFAULT_REGION, OSS_BASE_URL};

// #[allow(unused)]
// #[derive(Debug)]
// struct Options<'a> {
//     pub access_key_id: &'a str,
//     pub access_key_secret: &'a str,
//     pub sts_token: &'a str,
//     pub bucket: &'a str,
//     pub endpoint: &'a str,
//     pub region: &'a str,
//     pub internal: bool,
//     pub cname: bool,
//     pub is_request_pay: bool,
//     pub secure: bool,
//     pub timeout: u64,
// }

// impl<'a> Default for Options<'a> {
//     fn default() -> Self {
//         Self {
//             access_key_id: Default::default(),
//             access_key_secret: Default::default(),
//             sts_token: Default::default(),
//             bucket: Default::default(),
//             endpoint: Default::default(),
//             region: DEFAULT_REGION,
//             internal: false,
//             cname: true,
//             is_request_pay: false,
//             secure: true,
//             timeout: 60,
//         }
//     }
// }

// #[allow(unused)]
// impl<'a> Options<'a> {
//     fn root_url(&self) -> String {
//         format!("{}://{}.{}", self.schema(), self.region, OSS_BASE_URL)
//     }

//     fn base_url(&self) -> String {
//         format!(
//             "{}://{}.{}.{}",
//             self.schema(),
//             self.bucket,
//             self.region,
//             OSS_BASE_URL
//         )
//     }

//     fn schema(&self) -> &'a str {
//         if self.secure == true {
//             "https"
//         } else {
//             "http"
//         }
//     }
// }

// #[allow(unused)]
// #[derive(Debug)]
// struct OssClient<'a> {
//     options: Options<'a>,
//     client: Client,
// }

// impl<'a> OssClient<'a> {
//     #[allow(unused_mut)]
//     fn new(option: &'a Options) -> Self {
//         let mut opt = Options { ..*option };
//         let client = OssClient {
//             options: opt,
//             client: reqwest::Client::new(),
//         };
//         client
//     }

// }

// #[allow(unused)]
// pub struct OssRequest {
//     options: OssOptions,
//     client: Client,
//     method: Option<Method>,
//     url: Option<String>,
//     object_key: Option<String>,
//     headers: Option<HeaderMap>,
//     res: Option<String>,
//     data: Option<Bytes>,
// }

// impl Default for OssRequest {
//     fn default() -> Self {
//         Self {
//             options: OssOptions::default(),
//             client: reqwest::Client::builder().build().unwrap(),
//             method: Some(Method::GET),
//             url: None,
//             object_key: None,
//             headers: None,
//             res: None,
//             data: None
//         }
//     }
// }

use std::time::Duration;

use reqwest::{header::HeaderMap, Client};

#[allow(unused)]
#[derive(Debug)]
pub struct OssRequest<'a> {
    access_key_id: &'a str,
    access_key_secret: &'a str,
    sts_token: &'a str,
    timeout: u64,
    inner: Client,
}

impl<'a> Default for OssRequest<'a> {
    fn default() -> Self {
        let default_headers = HeaderMap::new();
        let client = Client::builder()
            .default_headers(default_headers)
            .user_agent(Self::USER_AGENT)
            .connect_timeout(Duration::from_secs(Self::DEFAULT_CONNECT_TIMEOUT))
            .build()
            .unwrap();
        Self {
            access_key_id: Default::default(),
            access_key_secret: Default::default(),
            sts_token: Default::default(),
            timeout: 60,
            inner: client,
        }
    }
}

#[allow(unused)]
impl<'a> OssRequest<'a> {
    const OSS_BASE_URL: &'static str = "aliyuncs.com";
    const DEFAULT_REGION: &'static str = "oss-cn-hangzhou";
    const USER_AGENT: &'static str = "xt oss/0.1";
    const DEFAULT_CONNECT_TIMEOUT: u64 = 60;

    fn new() -> Self {
        Self::default()
    }

    fn access_key_id(mut self, value: &'a str) -> Self {
        self.access_key_id = value;
        self
    }

    fn access_key_secret(mut self, value: &'a str) -> Self {
        self.access_key_secret = value;
        self
    }

    fn sts_token(mut self, value: &'a str) -> Self {
        self.sts_token = value;
        self
    }

    fn timeout(mut self, value: u64) -> Self {
        self.timeout = value;
        self
    }
}

#[tokio::main]
async fn main() {
    let req = OssRequest::new()
        .access_key_id("abcd")
        .access_key_secret("1234")
        .timeout(32)
        .sts_token("abcd");

    println!("{:#?}",req);

    let req1 = OssRequest::new();

    println!("{:#?}",req1);
}
