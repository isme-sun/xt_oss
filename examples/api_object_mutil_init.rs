//! `cargo run --example api_object_mutil_init -q`
//!
//! 使用Multipart Upload模式传输数据前,您必须先调用InitiateMultipartUpload接口来通知OSS初始化一
//! 个Multipart Upload事件。
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/initiatemultipartupload)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_object_mutil_init.rs)
use dotenv;
use std::process;
use xt_oss::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);
    match client
        .InitiateMultipartUpload("tmp/test1.png")
        .with_content_type("image/png")
        .execute()
        .await
        .unwrap_or_else(|reqwest_error| {
            eprintln!("{}", reqwest_error);
            process::exit(-1);
        }) {
        Ok(oss_data) => {
            println!("{:#?}", oss_data.content())
        }
        Err(error_message) => {
            println!("{:#?}", error_message.content())
        }
    }
    Ok(())
}
