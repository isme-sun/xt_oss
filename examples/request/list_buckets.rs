use std::env;
use xt_oss::oss::Request;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let access_key_id = env::var("OSS_ACCESS_KEY_ID").unwrap();
    let access_key_secret = env::var("OSS_ACCESS_KEY_SECRET").unwrap();
    let url = "https://oss-cn-hangzhou.aliyuncs.com";

    let resp = Request::new()
        .with_access_key_id(&access_key_id)
        .with_access_key_secret(&access_key_secret)
        .task()
        .with_url(&url)
        // default Method::GET
        // .with_method(http::Method::GET)
        .execute()
        .await;

    match resp {
        Ok(resp) => {
            let bytes = resp.bytes().await.unwrap();
            let content = String::from_utf8_lossy(&bytes);
            println!("{}", content);
        }
        Err(error) => {
            println!("reqwest error: {}", error)
        }
    }
}
