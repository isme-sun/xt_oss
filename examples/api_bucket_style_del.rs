//! `cargo run --package xt-oss --example api_bucket_style_del`
//!
//! 调用DeleteStyle删除某个Bucket下指定的图片样式
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/deletestyle)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_style_del.rs)
use dotenv;
use std::process;
use xt_oss::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);
    let result = client
        .DeleteStyle("cover")
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
    Ok(())
}
