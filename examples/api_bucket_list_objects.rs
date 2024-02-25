//! `cargo run --example api_bucket_list_objects -q`
//!
//! GetBucket (ListObjects)接口用于列举存储空间`Bucket`中所有文件
//! `Object`的信息。
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/listobjects)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_stand_list_object.rs)
use dotenv;
use std::process;
use xt_oss::prelude::*;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);
    let result = client
        .ListObjects()
        // .with_max_keys(20)
        // .with_prefix("course/video")
        // .with_marker("course/video/00518bfd279de57ea6a8b26a5af2c0fc/content.mp4")
        .execute()
        .await
        .unwrap_or_else(|error| {
            println!("reqwest error: {}", error);
            process::exit(-1);
        });

    match result {
        Ok(data) => {
            println!("{:#?}", data.content())
        }
        Err(error) => {
            println!("{:#?}", error.content())
        }
    }
}
