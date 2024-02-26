//! `cargo run --package xt-oss --example api_bucket_transfer_acceleration_put`
//!
//! 接口用于为存储空间（Bucket）配置传输加速。开启传输加速后，可提升全球各地用户对OSS的访问速度，
//! 适用于远距离数据传输、GB或TB级大文件上传和下载的场景。
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/putbuckettransferacceleration)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_transfer_acceleration_put.rs)
use dotenv;
use std::process;
use xt_oss::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    match util::options_from_env()
        .client()
        // .PutBucketTransferAcceleration(true)
        .PutBucketTransferAcceleration(false)
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
