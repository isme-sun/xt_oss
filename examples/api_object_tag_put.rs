//! `cargo run --example api_object_tag_put -q`
//!
//! 调用PutObjectTagging接口设置或更新对象`Object`的标签`Tagging`信息。
//! 对象标签使用一组键值对`Key-Value`标记对象。
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/putobjecttagging)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_object_tag_put.rs)
use dotenv;
use std::process;
use xt_oss::prelude::*;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);

    match client
        .PutObjectTagging("excel/Spreadsheet-1000-rows.xls")
        .with_tags(vec![
            ("key1", "value1"),
            ("key2", "value2"),
            ("key3", "value3"),
        ])
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
