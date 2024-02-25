//! `cargo run --example api_object_stand_meta -q`
//!
//! 调用GetObjectMeta接口获取一个文件`Object`的元数据信息
//! 包括该Object的ETag、Size、LastModified信息,并且不返回该Object的内容。
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/getobjectmeta)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_object_stand_meta.rs)
use dotenv;
use std::process;
use xt_oss::prelude::*;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);
    let resp = client
        .GetObjectMeta("mp3/Audio_0.4mb.mp3")
        // .with_version_id(version_id)
        // .with_match(value)
        // .with_modified_since(value)
        // .with_none_match(value)
        // .with_unmodified_since(value)
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
