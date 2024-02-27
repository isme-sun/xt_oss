//! `cargo run --example api_object_stand_head -q`
//!
//! HeadObject接口用于获取某个文件`Object`的元信息
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/headobject)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_object_stand_head.rs)
use dotenv;
use std::process;
use xt_oss::prelude::*;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);

    let resp = client
        .HeadObject("mp3/Audio_0.4mb.mp3")
        // .with_version_id("version_id")
        .execute()
        .await
        .unwrap_or_else(|reqwest_error| {
            println!("reqwest error: {}", reqwest_error);
            process::exit(-1);
        });
    match resp {
        Ok(data) => {
            println!("{:#?}", data.headers())
        }
        Err(error_message) => {
            println!("{:#?}", error_message.content())
        }
    }
}
