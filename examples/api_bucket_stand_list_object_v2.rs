//! `cargo run --package xt-oss --example api_bucket_stand_list_object_v2`
//!
//! ListObjectsV2`GetBucketV2`接口用于列举存储空间`Bucket`中所有文件
//! `Object`的信息。
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/listobjectsv2)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_stand_list_object_v2.rs)
use dotenv;
use std::process;
use xt_oss::{
    prelude::*,
    util,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);
    match client
        .ListObjectsV2()
        .with_max_keys(10)
        .with_delimiter("/")
        .execute()
        .await
        .unwrap_or_else(|error| {
            println!("reqwest error: {}", error);
            process::exit(-1);
        }) {
        Ok(oss_data) => {
            let result = oss_data.content();
            println!("{:#?}", result);
        }
        Err(oss_error_message) => {
            println!("oss error message: {}", oss_error_message.content())
        }
    }
    Ok(())
}
