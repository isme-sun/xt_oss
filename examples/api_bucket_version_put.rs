//!  `cargo run --package xt-oss --example api_bucket_put_versioning`
//!
//! 调用PutBucketVersioning设置指定存储空间`Bucket`的版本控制状态。
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/putbucketversioning)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_version_put.rs)
use dotenv;
use std::process;
use xt_oss::{oss::entities::version::VersioningStatus, prelude::*};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    match util::options_from_env()
        .client()
        // .PutBucketVersioning(VersioningStatus::Enabled)
        .PutBucketVersioning(VersioningStatus::Suspended)
        .execute()
        .await
        .unwrap_or_else(|error| {
            println!("reqwest error {}", error);
            process::exit(-1);
        }) {
        Ok(_) => {
            println!("success")
        }
        Err(oss_error_message) => {
            println!("{:#?}", oss_error_message.content());
        }
    };
    Ok(())
}
