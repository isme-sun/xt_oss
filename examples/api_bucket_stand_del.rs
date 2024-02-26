//! `cargo run --package xt-oss --example api_bucket_stand_del`
//!
//! 调用DeleteBucket删除某个存储空间`Bucket`。
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/deletebucket)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_stand_del.rs)
use dotenv;
use std::process;
use xt_oss::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);
    let result = client
        .DeleteBucket()
        // .with_region("oss-cn-beijing")
        // .with_bucket("xtoss-t1")
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
        Err(oss_error) => {
            println!("oss error:{}", oss_error.content())
        }
    }
    Ok(())
}
