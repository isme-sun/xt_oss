//! `cargo run --example api_bucket_logging_del -q`
//!
//! DeleteBucketLogging用于关闭存储空间`Bucket`的访问日志记录功能。
//! 只有Bucket的拥有者才有权限关闭Bucket访问日志记录功能
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/deletebucketlogging)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_logging_del.rs)
use dotenv;
use std::process;
use xt_oss::prelude::*;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);
    let result = client
        .DeleteBucketLogging()
        .execute()
        .await
        .unwrap_or_else(|reqwest_error| {
            println!("reqwest error: {}", reqwest_error);
            process::exit(-1);
        });

    match result {
        Ok(oss_data) => {
            println!("{:#?}", &oss_data.headers());
        }
        Err(error_message) => {
            println!("{}", error_message.content())
        }
    }
}
