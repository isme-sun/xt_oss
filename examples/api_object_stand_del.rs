//! `cargo run --example api_object_stand_del -q`
//! 调用DeleteObject删除某个文件`Object`
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/deleteobject)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_object_stand_del.rs)
use dotenv;
use std::process;
use xt_oss::prelude::*;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);
    match client
        .DeleteObject("tmp/test.txt")
        .with_version_id("CAEQ2AEYgYCA1v6ot.sYIiBmZjU2NTQwOGEwZDc0MTMyYTU5ZjhlMmUyNGYwMjc3NA--")
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
}
