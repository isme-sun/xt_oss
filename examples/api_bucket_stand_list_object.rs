//! cargo run --package xt-oss --example api_bucket_stand_list_object 
//!
//! GetBucket (ListObjects)接口用于列举存储空间`Bucket`中所有文件
//! `Object`的信息。
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/listobjects)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_stand_list_object.rs)
use dotenv;
use serde_json;
use std::process;
use xt_oss::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);
    match client
        .ListObjects()
        .with_delimiter("/")
        // .with_encoding_type("url")
        // .with_max_keys(20)
        // .with_prefix("prefix")
        .execute()
        .await
        .unwrap_or_else(|reqwest_error| {
            eprint!("reqwest error: {}", reqwest_error);
            process::exit(-1);
        }) {
        Ok(oss_data) => {
            let result = oss_data.content();
            println!("{}", serde_json::to_string_pretty(&result).unwrap());
        }
        Err(error_message) => println!("oss error: {}", error_message.content()),
    }
    Ok(())
}
