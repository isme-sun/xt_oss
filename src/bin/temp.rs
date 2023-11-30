use std::time::Duration;

use bytes::Bytes;
use reqwest::{
    header::{HeaderMap, CONTENT_TYPE},
    Client, Method,
};
use xt_oss::OssData;

#[allow(unused)]
#[derive(Debug)]
pub struct OssRequest<'a> {
    access_key_id: &'a str,
    access_key_secret: &'a str,
    sts_token: &'a str,
    timeout: u64,
    client: Client,
    method: Method,
    headers: Option<HeaderMap>,
    resourse: Option<&'a str>,
    body: Option<Bytes>,
}

impl<'a> Default for OssRequest<'a> {
    fn default() -> Self {
        let mut default_headers = HeaderMap::new();
        default_headers.insert(CONTENT_TYPE, "application/octet-stream".parse().unwrap());
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
            client,
            method: Method::GET,
            headers: None,
            resourse: None,
            body: None,
        }
    }
}

#[allow(unused)]
impl<'a> OssRequest<'a> {
    const OSS_BASE_URL: &'static str = "aliyuncs.com";
    const DEFAULT_REGION: &'static str = "oss-cn-hangzhou";
    const USER_AGENT: &'static str = "xt oss/0.1";
    const DEFAULT_CONTENT_TYPE: &'static str = "application/octet-stream";
    const DEFAULT_CONNECT_TIMEOUT: u64 = 180;

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

    fn method(mut self, value: Method) -> Self {
        self.method = value;
        self
    }

    fn headers(mut self, value: HeaderMap) -> Self {
        self.headers = Some(value);
        self
    }

    fn resource(mut self, value: &'a str) -> Self {
        self.resourse = Some(value);
        self
    }

    fn body(mut self, value: Bytes) -> Self {
        self.body = Some(value);
        self
    }

    async fn execute(mut self, url: &'a str) {
        // let headers = self.headers.unwrap_or(HeaderMap::new());
        let request_builder = self
            .client
            .request(self.method, url)
            .headers(self.headers.unwrap_or(HeaderMap::new()))
            .body(self.body.unwrap_or(Bytes::new()));

        let resp = request_builder.send().await.unwrap();
        let status = resp.status();
        let headers = resp.headers().clone();
        let data = resp.bytes().await.unwrap();
        let oss_data = OssData {
            // status,
            headers,
            data,
        };
        println!("{:#?}", oss_data);
    }
}

#[tokio::main]
async fn main() {
    let url = "https://dev-service.xuetube.com/api/system/echo?name=sjy";
    let mut headers = HeaderMap::new();
    headers.insert("x-name", "xuetube".parse().unwrap());
    headers.insert("x-1", "xuetube".parse().unwrap());
    headers.insert("x-2", "xuetube".parse().unwrap());

    OssRequest::new().headers(headers).execute(url).await;
}
