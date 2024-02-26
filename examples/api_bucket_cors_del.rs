//! `cargo run --example api_bucket_cors_del -q`
//!
//! DeleteBucketCors用于关闭指定存储空间`Bucket`对应的跨域资源共享CORS
//! `Cross-Origin Resource Sharing`功能并清空所有规则
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/deletebucketcors)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_cors_del.rs)
use dotenv;
use std::process;
use xt_oss::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);
    let result = client
        .DeleteBucketCors()
        .execute()
        .await
        .unwrap_or_else(|reqwest_error| {
            println!("reqwest error: {}", reqwest_error);
            process::exit(-1);
        });

    match result {
        Ok(oss_data) => {
            println!("{:#?}", oss_data.headers())
        }
        Err(error_message) => {
            println!("{}", error_message.content())
        }
    }
    Ok(())
}
