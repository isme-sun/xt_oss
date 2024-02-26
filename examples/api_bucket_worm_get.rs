//! `cargo run --example api_bucket_worm_get -q`
//! 
//! GetBucketWorm用于获取指定存储空间`Bucket`的合规保留策略信息。
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/getbucketworm)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_worm_get.rs)
use dotenv;
use std::process;
use xt_oss::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);
    let result = client
        .GetBucketWorm()
        .execute()
        .await
        .unwrap_or_else(|reqwest_error| {
            println!("reqwest {}", reqwest_error);
            process::exit(-1);
        });
    match result {
        Ok(oss_data) => {
            println!("{:#?}", oss_data.content());
        }
        Err(error_message) => {
            println!("oss error: {}", error_message.content());
        }
    }

    Ok(())
}
