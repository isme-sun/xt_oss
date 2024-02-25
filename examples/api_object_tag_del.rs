//! `cargo run --example api_object_tag_del -q`
//!
//! 调用DeleteObjectTagging接口删除指定对象`Object`的标签`Tagging`信息。
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/deleteobjecttagging)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_object_tag_del.rs)
use dotenv;
use std::process;
use xt_oss::prelude::*;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);

    match client
        .DeleteObjectTagging("excel/Spreadsheet-1000-rows.xls")
        .execute()
        .await
        .unwrap_or_else(|error| {
            println!("{}", error);
            process::exit(-1);
        }) {
        Ok(data) => {
            println!("{:#?}", data.headers())
        }
        Err(message) => {
            println!("{:#?}", message.content())
        }
    }
}
