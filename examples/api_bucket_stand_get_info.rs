//! `cargo run --package xt-oss --example api_bucket_stand_get_info`
//!
//! 调用GetBucketInfo接口查看存储空间`Bucket`的相关信息。
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/getbucketinfo)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_stand_get_info.rs)
use dotenv;
use serde_json;
use std::process;
use xt_oss::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    // dbg!(&options);
    let client = oss::Client::new(options);
    let result = client
        .GetBucketInfo()
        // .with_bucket("xtoss-ex1")
        .execute()
        .await
        .unwrap_or_else(|error| {
            println!("reqwest error: {}", error);
            process::exit(-1);
        });

    match result {
        Ok(oss_data) => {
            println!(
                "{}",
                serde_json::to_string_pretty(&oss_data.content()).unwrap()
            );
        }
        Err(oss_error_message) => {
            println!("{:#?}", oss_error_message.content())
        }
    }
    Ok(())
}
