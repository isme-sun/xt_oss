//! `cargo run --package xt-oss --example api_bucket_version_get `
//!
//! 接口用于获取指定Bucket的版本控制状态。
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/getbucketversioning)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_version_get.rs)
use dotenv;
use std::process;
use xt_oss::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    match util::options_from_env()
        .client()
        .GetBucketVersioning()
        .execute()
        .await
        .unwrap_or_else(|error| {
            println!("reqwest error {}", error);
            process::exit(-1);
        }) {
        Ok(oss_data) => {
            let version_config = oss_data.content();
            if let Some(status) = version_config.status {
                println!("version status: {}", status);
            } else {
                println!("Version feature not enabled");
            }
        }
        Err(oss_error_message) => {
            println!("{:#?}", oss_error_message.content());
        }
    };
    Ok(())
}
