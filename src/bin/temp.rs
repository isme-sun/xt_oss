#[allow(unused_imports)]
use base64::{engine::general_purpose, Engine as _};
use chrono::{DateTime, Utc};
use hmacsha1;
use std::str::FromStr;
#[allow(unused_imports)]
use std::{str::from_utf8, time::Duration};

use bytes::Bytes;
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Client, IntoUrl, Method, Url,
};
use xt_oss::OssData;

#[allow(unused)]
#[derive(Debug)]
pub struct OssRequest<'a> {
    access_key_id: Option<&'a str>,
    access_key_secret: Option<&'a str>,
    sts_token: Option<&'a str>,
    timeout: u64,
    client: Client,
    bucket: Option<&'a str>,
    object: Option<&'a str>,
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
            access_key_id: None,
            access_key_secret: None,
            sts_token: None,
            timeout: 60,
            client,
            method: Method::GET,
            bucket: None,
            object: None,
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
        self.access_key_id = Some(value);
        self
    }

    fn access_key_secret(mut self, value: &'a str) -> Self {
        self.access_key_secret = Some(value);
        self
    }

    fn sts_token(mut self, value: &'a str) -> Self {
        self.sts_token = Some(value);
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

    fn bucket(mut self, value: &'a str) -> Self {
        self.bucket = Some(value);
        self
    }

    fn object(mut self, value: &'a str) -> Self {
        self.object = Some(value);
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
            .header(AUTHORIZATION, HeaderValue::from_static("test"))
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

    fn authorization(mut self) -> String {
        let auth = if let (Some(key), Some(secret)) = (self.access_key_id, self.access_key_secret) {
            format!("OSS {}:{}", key, secret)
        } else {
            "".into()
        };
        auth
    }

    fn signature(mut self, dt: DateTime<Utc>) -> String {
        let fmt = "%a, %d %b %Y %H:%M:%S GMT";
        let value = format!(
            "{VERB}\n\n{ContentType}\n{Date}\n{Resource}",
            VERB = &self.method.to_string(),
            ContentType= OssRequest::DEFAULT_CONNECT_TIMEOUT,
            Date = dt.format(fmt).to_string(),
            Resource = &self.canonicalized_resource()
        );
        let key = self.access_key_secret.unwrap().as_bytes();
        let message = &value.as_bytes();
        let value = hmacsha1::hmac_sha1(key,message);
        general_purpose::STANDARD.encode(value.as_slice())
    }

    fn canonicalized_resource(&self) -> String {
        let res_path = match (self.bucket, self.object) {
            (Some(bucket), Some(object_key)) => {
                format!("/{}/{}", bucket, object_key)
            }
            (Some(bucket), None) => {
                format!("/{}/", bucket)
            }
            (None, None) => "/".to_string(),
            (None, Some(_)) => {
                panic!("params error")
            }
        };
        if let Some(res) = &self.resourse {
            format!("{}?{}", res_path, res)
        } else {
            format!("{}", res_path)
        }
    }
}

#[derive(Debug, Default)]
#[allow(unused)]
pub struct OssUrl<T: IntoUrl> {
    origin: T,
    bucket: String,
    region: String,
    object: String,
    res: String,
}

impl<T> OssUrl<T>
where
    T: IntoUrl,
{
    pub fn from(value: T) -> Self {
        // let url = value.try_into::<Url>().unwrap();
        Self {
            origin: value,
            bucket: "".into(),
            region: "".into(),
            object: "".into(),
            res: "".into(),
        }
    }
}

#[tokio::main]
async fn main() {
    // let content = "PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iVVRGLTgiPz4KPEVycm9yPgogIDxDb2RlPkFjY2Vzc0RlbmllZDwvQ29kZT4KICA8TWVzc2FnZT5Zb3UgaGF2ZSBubyByaWdodCB0byBhY2Nlc3MgdGhpcyBvYmplY3QgYmVjYXVzZSBvZiBidWNrZXQgYWNsLjwvTWVzc2FnZT4KICA8UmVxdWVzdElkPjY1NjgwRjc3MzcxRjE0MzkzMzJFMUNDMjwvUmVxdWVzdElkPgogIDxIb3N0SWQ+eHVldHViZS1kZXYub3NzLWNuLXNoYW5naGFpLmFsaXl1bmNzLmNvbTwvSG9zdElkPgogIDxFQz4wMDAzLTAwMDAwMDAxPC9FQz4KICA8UmVjb21tZW5kRG9jPmh0dHBzOi8vYXBpLmFsaXl1bi5jb20vdHJvdWJsZXNob290P3E9MDAwMy0wMDAwMDAwMTwvUmVjb21tZW5kRG9jPgo8L0Vycm9yPgo=";
    // let rs = general_purpose::STANDARD.decode(content).unwrap();

    // println!("{}", String::from_utf8_lossy(&rs));

    // * ------------------------------------------------------------------------------------

    // 从一个地址中解析 bucket | region | object | res
    let url =
        "https://xuetube-dev.oss-cn-shanghai.aliyuncs.com/course/content-400x400.jpeg?objectMeta";

    let url = Url::from_str(url).unwrap();
    println!("{}", url.domain().as_ref().unwrap());
    println!("{}", url.host().as_ref().unwrap());

    // let oss_req = OssRequest::new()
    //                 .bucket("xuetube-dev")
    //                 .object("index.html")
    //                 .method(Method::HEAD)
    //                 .resource("objectMeta");

    // println!("{}",oss_req.canonicalized_resource());

    // let resp = oss_req.execute(url).await;
    // println!("{:#?}", resp);
}
