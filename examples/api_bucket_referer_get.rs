//! `cargo run --package xt-oss --example api_bucket_referer_get`
//!
//! GetBucketReferer接口用于查看存储空间`Bucket`的防盗链`Referer`相关配置。
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/getbucketreferer)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_referer_get.rs)
use dotenv;
use std::process;
use xt_oss::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);
    let result = client
        .GetBucketReferer()
        .execute()
        .await
        .unwrap_or_else(|reqwest_error| {
            println!("reqwest error: {}", reqwest_error);
            process::exit(-1);
        });

    match result {
        Ok(oss_data) => {
            println!("{:#?}", oss_data.content());
        }
        Err(error_message) => {
            println!("oss error:{}", error_message.content())
        }
    }
    Ok(())
}
