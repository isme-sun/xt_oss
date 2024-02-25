//! `cargo run --example api_bucket_logging_put -q`
//!
//! PutBucketLogging接口用于为存储空间`Bucket`开启日志转存功能，
//! 可将OSS的访问日志按照固定命名规则,以小时为单位生成日志文件写入您
//! 指定的Bucket。
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/putbucketlogging)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_logging_put.rs)
use dotenv;
use std::process;
use xt_oss::prelude::*;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);
    let result = client
        .PutBucketLogging()
        .with_enabled(true)
        .with_bucket("xtoss-ex11")
        .with_target_prefix("ex10-")
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
        Err(error_message) => {
            println!("{}", error_message.content())
        }
    }
}
