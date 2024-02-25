//! `cargo run --example api_object_symlink_get -q`
//!
//! 调用GetSymlink接口获取软链接。此操作需要您对该软链接有读权限。
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/getsymlink)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_object_symlink_get.rs)
use dotenv;
use std::process;
use xt_oss::prelude::*;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);

    match client
        .GetSymlink("tmp/test.xls")
        .execute()
        .await
        .unwrap_or_else(|error| {
            println!("{}", error);
            process::exit(-1);
        }) {
        Ok(oss_data) => {
            println!("{:#?}", &oss_data.headers())
        }
        Err(error_message) => {
            println!("{:#?}", &error_message.content())
        }
    }
}
