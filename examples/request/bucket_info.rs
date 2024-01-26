use std::env;
use reqwest::Response;
use xt_oss::oss::{self, http, Request};
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
        .with_resource("/xuetube-dev/?bucketInfo")
        .with_method(http::Method::GET)
        .execute_timeout(30)
        .await;

    match resp {
        Ok(resp) => {
            let bytes: oss::Bytes = resp.bytes().await.unwrap();
            let content = String::from_utf8_lossy(&bytes);
            println!("{}", content);
        }
        Err(error) => {
            println!("{:#?}", error);
        }
    }
}
