//!  `cargo run --example api_bucket_logging_get -q`
//!
//! GetBucketLogging接口用于查看存储空间`Bucket`的访问日志配置。
//! 只有Bucket的拥有者才能查看Bucket的访问日志配置。
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/getbucketlogging)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_logging_get.rs)
use dotenv;
use std::process;
use xt_oss::prelude::*;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);
    let result = client
        .GetBucketLogging()
        .execute()
        .await
        .unwrap_or_else(|reqwest_error| {
            println!("reqwest error: {}", reqwest_error);
            process::exit(-1);
        });

    match result {
        Ok(oss_data) => {
            println!(
                "{}",
                serde_json::to_string_pretty(&oss_data.content()).unwrap()
            );
        }
        Err(error_message) => {
            println!("{}", error_message.content())
        }
    }
}
