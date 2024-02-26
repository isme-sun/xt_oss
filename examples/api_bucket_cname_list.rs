//! `cargo run --example api_bucket_cname_list -q`
//!
//! 调用ListCname接口用于查询某个存储空间(Bucket)下绑定的所有的自定义域名(Cname)列表
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/listcname)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_cname_list.rs)
use dotenv;
use std::process;
use xt_oss::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);
    let result = client
        .ListCname()
        .execute()
        .await
        .unwrap_or_else(|reqwest_error| {
            println!("reqwest error: {}", reqwest_error);
            process::exit(-1);
        });
    match result {
        Ok(oss_data) => {
            println!(
                "{}",
                serde_json::to_string_pretty(&oss_data.content()).unwrap()
            );
        }
        Err(error_message) => {
            println!("oss error: {}", error_message.content())
        }
    }
    Ok(())
}
