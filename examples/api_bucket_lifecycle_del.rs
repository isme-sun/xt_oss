//!  cargo run --example api_bucket_lifecycle_del -q
//!
//! DeleteBucketLifecycle接口用于删除指定存储空间`Bucket`的生命周期规则。
//! 使用DeleteBucketLifecycle接口删除指定Bucket所有的生命周期规则后,
//! 该Bucket中的文件`Object`不会被自动删除。只有Bucket的拥有者才能删除该Bucket
//! 的生命周期规则。
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/deletebucketlifecycle)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_lifecycle_del.rs)
use std::process;

use xt_oss::prelude::*;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);

    match client
        .DeleteBucketLifecycle()
        .execute()
        .await
        .unwrap_or_else(|reqwest_error| {
            println!("{}", reqwest_error);
            process::exit(-1);
        }) {
        Ok(oss_data) => {
            println!("{:#?}", oss_data.headers())
        }
        Err(error_message) => {
            println!("{:#?}", error_message.headers())
        }
    }
}
