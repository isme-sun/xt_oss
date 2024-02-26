//! `cargo run --package xt-oss --example api_bucket_style_get`
//!
//! 调用GetStyle接口查询某个Bucket下指定的样式信息
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/getstyle)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_style_list.rs)
use dotenv;
use std::process;
use xt_oss::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);
    let result = client
        .GetStyle("avatar")
        .execute()
        .await
        .unwrap_or_else(|reqwest_error| {
            println!("reqwest error: {}", reqwest_error);
            process::exit(-1);
        });

    match result {
        Ok(oss_data) => {
            println!("{:#?}", oss_data.content());
        }
        Err(error_message) => {
            println!("{}", error_message.content())
        }
    }
    Ok(())
}
