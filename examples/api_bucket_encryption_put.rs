//! `cargo run --example api_bucket_encryption_put -q`
//!
//! PutBucketEncryption接口用于配置存储空间`Bucket`的加密规则。
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/putbucketencryption)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_encryption_put.rs)
use dotenv;
use std::process;
use xt_oss::{oss::entities::encryption::SSEAlgorithm, prelude::*};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);
    let result = client
        .PutBucketEncryption()
        .with_algorithm(SSEAlgorithm::KMS)
        // .with_data_encryption("SM4")
        // .with_master_key_id("--your value --")
        .execute()
        .await
        .unwrap_or_else(|reqwest_error| {
            println!("reqwest error: {}", reqwest_error);
            process::exit(-1);
        });

    match result {
        Ok(oss_data) => {
            println!("{:#?}", oss_data.headers())
        }
        Err(error_message) => {
            println!("{}", error_message.content())
        }
    }
    Ok(())
}
