//! `cargo run --example api_object_mutil_list -q`
//!
//! 调用ListMultipartUploads接口列举所有执行中的Multipart Upload事件,即已经初始化但还未完成
//! `Complete`或者还未中止`Abort`的Multipart Upload事件。
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/listmultipartuploads)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_object_mutil_list.rs)
use dotenv;
use std::process;
use xt_oss::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);
    match client
        .ListMultipartUploads()
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
