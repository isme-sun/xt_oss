//! `cargo run --example api_bucket_worm_complete -q`
//! 
//! CompleteBucketWorm用于锁定合规保留策略。
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/completebucketworm)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_worm_complete.rs)
use dotenv;
use std::process;
use xt_oss::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);
    match client
        .CompleteBucketWorm("933141599A8941FD9592F24F9862A5DE")
        .execute()
        .await
        .unwrap_or_else(|error| {
            println!("reqwest error {}", error);
            process::exit(-1);
        }) {
        Ok(oss_data) => {
            println!("{:#?}", oss_data.content());
        }
        Err(oss_error_message) => {
            println!("{:#?}", oss_error_message.content());
        }
    };
    Ok(())
}
