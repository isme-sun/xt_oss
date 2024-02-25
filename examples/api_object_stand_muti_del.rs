//! `cargo run --example api_object_stand_muti_del -q`
//!
//! DeleteMultipleObjects接口用于删除同一个存储空间`Bucket`中的多个文件`Object`
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/deletemultipleobjects)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_object_stand_mutil_del.rs)
use dotenv;
use std::process;
use xt_oss::prelude::*;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);
    match client
        .DeleteMultipleObjects()
        .with_deletes(vec![
            ("images/JPGImage_100kbmb.jpg", ""),
            ("images/JPGImage_15mbmb.jpg", ""),
        ])
        .execute()
        .await
        .unwrap_or_else(|reqwest_error| {
            eprintln!("{}", reqwest_error);
            process::exit(-1);
        }) {
        Ok(oss_data) => {
            println!("{:#?}", oss_data.headers());
            println!("{:#?}", oss_data.content());
        }
        Err(error_message) => {
            println!("{:#?}", error_message.content())
        }
    }
}
