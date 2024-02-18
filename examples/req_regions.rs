pub(crate) use std::{env, process};

use xt_oss::oss;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let access_key_id = env::var("OSS_ACCESS_KEY_ID").unwrap();
    let access_key_secret = env::var("OSS_ACCESS_KEY_SECRET").unwrap();
    let url = "https://oss-cn-hangzhou.aliyuncs.com/?regions=oss-us-west-1";
    // let url = "https://oss-cn-hangzhou.aliyuncs.com/?regions";

    let resp = oss::Request::new()
        .with_access_key_id(&access_key_id)
        .with_access_key_secret(&access_key_secret)
        .task()
        .with_url(&url)
        // default Method::GET
        // .with_method(http::Method::GET)
        .execute_timeout(30)
        // default timeout = 60
        // .execute()
        .await
        .unwrap_or_else(|error| {
            println!("reqwest error: {}", error);
            process::exit(-1);
        });

    match resp.status().is_success() {
        true => println!("oss api sucess:"),
        false => println!("oss api fail:"),
    }

    println!("status: {}", resp.status());
    println!("headers: {:#?}", resp.headers());
    let data = resp.text().await.unwrap();
    println!("data: {}", data);
    Ok(())
}
