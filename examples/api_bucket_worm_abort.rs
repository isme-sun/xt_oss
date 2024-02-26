//! `cargo run --example api_bucket_worm_abort -q`
//! 
//! AbortBucketWorm用于删除未锁定的合规保留策略。
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/abortbucketworm)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_worm_abort.rs)
use dotenv;
use std::process;
use xt_oss::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);
    match client
        .AbortBucketWorm()
        .execute()
        .await
        .unwrap_or_else(|error| {
            println!("reqwest error {}", error);
            process::exit(-1);
        }) {
        Ok(oss_data) => {
            println!("{:#?}", oss_data.headers());
        }
        Err(oss_error_message) => {
            println!("{:#?}", oss_error_message.content());
        }
    };
    Ok(())
}
