//! `cargo run --example api_object_mutil_list_part -q`
//!
//! ListParts接口用于列举指定Upload ID所属的所有已经上传成功Part。
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/listparts)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_object_mutil_list_part.rs)
use dotenv;
use std::process;
use xt_oss::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);
    match client
        .ListParts("tmp/temp.jpg")
        .with_upload_id("E71E2C09F952430F93700A3167F74685")
        .execute()
        .await
        .unwrap_or_else(|reqwest_error| {
            eprintln!("reqwest error: {}", reqwest_error);
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
