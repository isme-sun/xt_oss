//! `cargo run --example api_bucket_lifecycle_get -q`
//! 
//! 调用GetBucketLifecycle接口查看存储空间`Bucket`的生命周期规则`Lifecycle`。
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/getbucketlifecycle)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_lifecycle_get.rs)
use std::process;

use xt_oss::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);

    match client
        .GetBucketLifecycle()
        .execute()
        .await
        .unwrap_or_else(|reqwest_error| {
            println!("{}", reqwest_error);
            process::exit(-1);
        }) {
        Ok(oss_data) => {
            println!("{:#?}", oss_data.content())
        }
        Err(error_message) => {
            println!("{:#?}", error_message.content())
        }
    }
    Ok(())
}
