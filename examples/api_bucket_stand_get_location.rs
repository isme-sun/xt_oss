//! `cargo run --package xt-oss --example api_bucket_stand_get_location`
//!
//! GetBucketLocation接口用于查看存储空间`Bucket`的位置信息。
//! 只有Bucket的拥有者才能查看Bucket的位置信息。
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/getbucketlocation)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_stand_get_location.rs)
use dotenv;
use std::process;
use xt_oss::{oss::entities::bucket::LocationConstraint, prelude::*};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);
    let result = client
        .GetBucketLocation()
        .with_bucket("xtoss-ex11")
        .execute()
        .await
        .unwrap_or_else(|reqwest_error| {
            println!("reqwest error: {}", reqwest_error);
            process::exit(-1);
        });

    match result {
        Ok(oss_data) => {
            let LocationConstraint(location) = oss_data.content();
            println!("location: {}", location);
        }
        Err(oss_error) => {
            println!("{}", oss_error.url());
            println!("{:#?}", oss_error.content())
        }
    }
    Ok(())
}
