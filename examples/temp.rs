use base64::{engine::general_purpose, Engine as _};
#[allow(unused)]
use bytes::Bytes;
#[allow(unused)]
use reqwest::{header::HeaderMap, Response, StatusCode};
#[allow(unused)]
use serde::Deserialize;
use std::process;
#[allow(unused)]
use std::{env, ops::Deref};
use url::Url;
#[allow(unused)]
use xt_oss::oss::{api::Message, entities::bucket::BucketInfo, http, Request};

// api 返回数据， 可能正确也可能错误
#[allow(unused)]
struct ApiData<T> {
    url: Url,
    status: StatusCode,
    headers: HeaderMap,
    content: T,
}

#[allow(unused)]
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

    pub fn content(&self) -> &T {
        &self.content
    }

    pub fn version_id(&self) -> String {
        self.headers
            .get("ok")
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    }
}

#[allow(unused)]
enum ApiResponse<T> {
    SUCCESS(ApiData<T>),
    FAIL(ApiData<Message>),
}

// api 返回体， 包含请求错误， 和api返回数据
#[allow(unused)]
type ApiResult<T> = Result<ApiResponse<T>, reqwest::Error>;

#[allow(unused)]
struct ApiResultFrom(Result<reqwest::Response, reqwest::Error>);

#[allow(unused)]
impl ApiResultFrom {
    async fn fail_message(resp: Response) -> ApiData<Message> {
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
        let content: Message = quick_xml::de::from_str(&content).unwrap();
        ApiData {
            url,
            status,
            headers,
            content,
        }
    }

    async fn bytes_data(resp: Response) -> ApiData<Bytes> {
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

    async fn to_type<T>(self) -> ApiResult<T>
    where
        T: for<'a> Deserialize<'a>,
    {
        let resp = self.0?;
        if resp.status().is_success() {
            let url = resp.url().clone();
            let status = resp.status().clone();
            let headers = resp.headers().clone();
            let content = resp.bytes().await.unwrap();
            let content = String::from_utf8_lossy(&content);
            let content: T = quick_xml::de::from_str(&content).unwrap();
            Ok(ApiResponse::SUCCESS(ApiData {
                url,
                status,
                headers,
                content,
            }))
        } else {
            let data_fail_message = Self::fail_message(resp).await;
            Ok(ApiResponse::FAIL(data_fail_message))
        }
    }

    async fn to_bytes(self) -> ApiResult<Bytes> {
        let resp = self.0?;
        if resp.status().is_success() {
            let data = Self::bytes_data(resp).await;
            Ok(ApiResponse::SUCCESS(data))
        } else {
            let data_fail_message = Self::fail_message(resp).await;
            Ok(ApiResponse::FAIL(data_fail_message))
        }
    }

    async fn to_empty(self) -> ApiResult<()> {
        let resp = self.0?;
        if resp.status().is_success() {
            Ok(ApiResponse::SUCCESS(ApiData {
                url: resp.url().clone(),
                status: resp.status().clone(),
                headers: resp.headers().clone(),
                content: (),
            }))
        } else {
            let data_fail_message = Self::fail_message(resp).await;
            Ok(ApiResponse::FAIL(data_fail_message))
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let access_key_id = env::var("OSS_ACCESS_KEY_ID").unwrap();
    let access_key_secret = env::var("OSS_ACCESS_KEY_SECRET").unwrap();
    // let url = "https://oss-cn-hangzhou.aliyuncs.com";
    // let url = "https://xuetube-dev.oss-cn-hangzhou.aliyuncs.com/?bucketInfo";
    let url = "https:/dev-cdn.xuetube.com/?bucketInfo";

    let resp = Request::new()
        .with_access_key_id(&access_key_id)
        .with_access_key_secret(&access_key_secret)
        .task()
        .with_url(&url)
        .with_resource("/xuetube-dev1/?bucketInfo")
        .with_method(http::Method::GET)
        .execute_timeout(10)
        .await;

    let result = ApiResultFrom(resp)
        .to_type::<BucketInfo>()
        .await
        .unwrap_or_else(|err| {
            println!("request error: {}", err);
            process::exit(-1);
        });

    match result {
        ApiResponse::SUCCESS(data) => {
            println!("{:#?}", data.content);
        }
        ApiResponse::FAIL(message) => {
            println!("{:#?}", message.content);
        }
    }

    // let resp = resp.map(|resp| ApiResponse { inner: resp });

    // match resp {
    //     Ok(resp) => {
    //         println!("api 返回");
    //         println!("{:#?}", resp);
    //     }
    //     Err(error) => {
    //         println!("api 请求错误");
    //         println!("{}", error);
    //     }
    // }
}
