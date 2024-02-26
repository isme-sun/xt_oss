//! `cargo run --example api_object_mutil_abort -q`
//! 
//! AbortMultipartUpload接口用于取消MultipartUpload事件并删除对应的Part数据。
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/abortmultipartupload)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_object_mutil_abort.rs)
use std::process;
use xt_oss::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);
    match client
        .AbortMultipartUpload("tmp/temp.jpg")
        .with_upload_id("EC83F9BA90DB4636BB26ECEAE205D6A8")
        .execute()
        .await
        .unwrap_or_else(|reqwest_error| {
            eprintln!("{}", reqwest_error);
            process::exit(-1);
        }) {
        Ok(oss_data) => {
            println!("{:#?}", oss_data.headers())
        }
        Err(error_message) => {
            println!("{:#?}", error_message.content())
        }
    }
    Ok(())
}
