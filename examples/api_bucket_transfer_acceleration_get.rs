//! `cargo run --package xt-oss --example api_bucket_transfer_acceleration_get `
//!
//! 接口用于获取目标存储空间（Bucket）的传输加速配置
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/getbuckettransferacceleration)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_transfer_acceleration_get.rs)
use dotenv;
use std::process;
use xt_oss::{oss::entities::acceleration::TransferAccelerationConfiguration, prelude::*};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    match util::options_from_env()
        .client()
        .GetBucketTransferAcceleration()
        // .PutBucketVersioning(VersioningStatus::Suspended)
        .execute()
        .await
        .unwrap_or_else(|error| {
            println!("reqwest error {}", error);
            process::exit(-1);
        }) {
        Ok(oss_data) => {
            let TransferAccelerationConfiguration { enabled } = oss_data.content();
            println!("enabled: {}", enabled);
        }
        Err(oss_error_message) => {
            println!("{:#?}", oss_error_message.content());
        }
    };
    Ok(())
}
