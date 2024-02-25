//! `cargo run --example api_bucket_list_objects_v2 -q`
//!
//! ListObjectsV2`GetBucketV2`接口用于列举存储空间`Bucket`中所有文件
//! `Object`的信息。
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/listobjectsv2)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/
use dotenv;
use std::process;
use xt_oss::prelude::*;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);
    let result = client
        .ListObjectsV2()
        .with_max_keys(10)
        // .with_prefix("course/video")
        .execute()
        .await
        .unwrap_or_else(|error| {
            println!("reqwest error: {}", error);
            process::exit(-1);
        });

    match result {
        Ok(data) => {
            println!("{}", serde_json::to_string_pretty(&data.content()).unwrap())
        }
        Err(error) => {
            println!("{:#?}", error.content())
        }
    }
}
