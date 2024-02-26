//! `cargo run --example api_bucket_worm_init -q`
//! 
//! 调用InitiateBucketWorm接口新建一条合规保留策略。
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/initiatebucketworm)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_worm_init.rs)
use dotenv;
use std::process;
use xt_oss::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);
    let result = client
        .InitiateBucketWorm()
        .with_days(1)
        .execute()
        .await
        .unwrap_or_else(|reqwest_error| {
            println!("reqwest {}", reqwest_error);
            process::exit(-1);
        });
    match result {
        Ok(oss_data) => {
            println!("{:#?}", oss_data.headers());
        }
        Err(error_message) => {
            println!("oss error: {}", error_message.content());
        }
    }

    Ok(())
}
