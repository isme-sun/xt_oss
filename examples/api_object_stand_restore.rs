//! `cargo run --example api_object_stand_restore -q`
//!
//! 调用RestoreObject接口解冻归档类型、冷归档、深度冷归档类型的文件`Object`
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/restoreobject)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_object_stand_restore.rs)
use dotenv;
use std::process;
use xt_oss::prelude::*;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);
    let resp = client
        .RestoreObject("mp3/Audio_0.4mb.mp3")
        .with_days(1)
        .execute()
        .await
        .unwrap_or_else(|error| {
            println!("reqwest error: {}", error);
            process::exit(-1);
        });
    match resp {
        Ok(data) => {
            println!("{:#?}", data.headers())
        }
        Err(message) => {
            println!("{:#?}", message.content())
        }
    }
}
