use std::{env, time::Duration};

use http::{
    self,
    header::{self, HOST},
    Method,
};
use reqwest;
use serde::{Serialize, Deserialize};
use xt_oss::OssError;

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
struct Regions {
    regions: Option<String>
}
// regions=oss-cn-hangzhou

#[allow(dead_code)]
#[allow(unused_variables)]
async fn oss_test() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().expect("error: .env not exist");

    let access_key_id = env::var("OSS_ACCESS_KEY_ID").unwrap_or_default();
    let access_key_secret = env::var("OSS_ACCESS_KEY_SECRET").unwrap_or_default();
    let oss_bucket = env::var("OSS_BUCKET").unwrap_or_default();

    let oss_base_url = "aliyuncs.com";
    let default_region = "oss-cn-hangzhou";

    let url = format!("https://{}.{}/", default_region, oss_base_url);

    let mut default_headers = header::HeaderMap::new();
    default_headers.insert(HOST, oss_base_url.parse().unwrap());

    // 创建一个缺省的client
    let client = reqwest::Client::builder()
        .default_headers(default_headers)
        .connect_timeout(Duration::from_secs(60))
        .build()
        .expect("error");

    // 获取 request 对象
    let request = client.request(Method::GET, url);

    // println!("{:#?}", request);

    let resp = request.send().await?;

    // if let Err(e) = resp {
    //     print!("{:?}", e);
    //     if e.is_status() {
    //         if let Some(status) = e.status() {
    //             println!("status {}", status);
    //         }
    //     } 
    // }

    // http::Error::from(resp.status());

    // println!("Headers:\n{:#?}", resp.headers());
    // println!("url:\n{:#?}", resp.url());
    let content  = resp.text().await?;
    let rs: OssError = serde_xml_rs::from_str(&content)?;
    // Err(anyhow!(rs))

    // let content  = resp.text().await?;
    // println!("{}", content);
    // let json_str:serde_json::Value  = serde_json::to_value(rs).unwrap();

    println!("{:#?}", rs);

    Ok(())
}

#[allow(dead_code)]
async fn req_test() -> Result<(), Box<dyn std::error::Error>> {
    // 访问地址
    let url = "https://service.xuetube.com/api/system/echo?status_code=403";
    // let url = "https://service.xuetube.com/123";
    let host = "service.xuetube.com";

    let mut default_headers = header::HeaderMap::new();
    default_headers.insert(HOST, host.parse().unwrap());

    // 创建一个缺省的client
    let client = reqwest::Client::builder()
        .default_headers(default_headers)
        .connect_timeout(Duration::from_secs(60))
        .build()
        .expect("error");

    // 获取 request 对象
    let request = client.request(Method::GET, url);

    println!("{:#?}", request);

    let resp = request.send().await?;

    println!("Status: {}", resp.status());
    println!("Headers:\n{:#?}", resp.headers());
    // println!("url:\n{:#?}", resp.url());
    // let content  = resp.text().await?;
    // println!("content:\n{:#?}", content);

    let json_str: serde_json::Value = resp.json().await?;

    println!("{}", json_str);
    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = oss_test().await {
        println!("----");
        println!("{:?}", e);
        println!("----");
        // println!("err: {}", err);
        // println!("is_builder {}", err.is_builder());
        // println!("is_body {}", err.is_body());
        // println!("is_connent {}", err.is_connect());
        // println!("is_decode {}", err.is_decode());
        // println!("is_redirect {}", err.is_redirect());
        // println!("is_request {}", err.is_request());
        // println!("is_status {}", err.is_status());
        // println!("is_timeout {}", err.is_timeout());
        // println!("status {:?}", err.status());
        // println!("url {:?}", err.url());
    }
}
