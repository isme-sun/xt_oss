//! `cargo run --package xt-oss --example api_bucket_stand_get_stat`
//!
//! 调用GetBucketStat接口获取指定存储空间`Bucket`的存储容量以及文件
//! `Object`数量
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/getbucketstat)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_stand_get_stat.rs)
use dotenv;
use serde_json;
use std::process;
use xt_oss::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);
    let result = client
        .GetBucketStat()
        .with_region("oss-cn-shanghai")
        .with_bucket("xtoss-ex10")
        .execute()
        .await
        .unwrap_or_else(|error| {
            println!("reqwest error: {}", error);
            process::exit(-1);
        });

    match result {
        Ok(data) => {
            println!("{}", serde_json::to_string_pretty(&data.content()).unwrap());
        }
        Err(error) => {
            println!("{}", error.url());
            println!("{:#?}", error.content())
        }
    }
    Ok(())
}
