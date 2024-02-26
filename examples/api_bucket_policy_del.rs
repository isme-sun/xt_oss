//! cargo run --package xt-oss --example api_bucket_policy_del 
//!
//! DeleteBucketPolicy用于删除指定存储空间`Bucket`的权限策略`Policy`。
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/deletebucketpolicy)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_policy_del.rs)
use dotenv;
use std::process;
use xt_oss::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);
    let result = client
        .DeleteBucketPolicy()
        .execute()
        .await
        .unwrap_or_else(|reqwest_error| {
            println!("reqwest error: {}", reqwest_error);
            process::exit(-1);
        });

    match result {
        Ok(oss_data) => {
            println!("{:#?}", oss_data.headers());
        }
        Err(error_message) => {
            println!("{}", error_message.content())
        }
    }
    Ok(())
}
