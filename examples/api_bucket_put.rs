//! `cargo run --package xt-oss --example api_bucket_put`
//!
//! 调用PutBucket接口创建存储空间`Bucket`。
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/putbucket)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_stand_put.rs)
use dotenv;
use std::process;

use xt_oss::{
    oss::entities::{OssAcl, StorageClass},
    prelude::*,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let options = util::options_from_env();
    let client = oss::Client::new(options);
    let result = client
        .PutBucket()
        .with_region("oss-cn-beijing")
        .with_bucket("xtoss-ex5")
        .with_acl(OssAcl::PublicRead)
        .with_storage_class(StorageClass::Standard)
        // .with_data_redundancy_type(DataRedundancyType::LRS)
        .execute()
        .await
        .unwrap_or_else(|error| {
            println!("reqwest error: {}", error);
            process::exit(-1);
        });

    match result {
        Ok(data) => {
            println!("{:#?}", data.headers())
        }
        Err(error) => {
            println!("{:#?}", error)
        }
    }
    Ok(())
}
