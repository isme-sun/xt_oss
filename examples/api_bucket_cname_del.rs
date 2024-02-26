//! `cargo run --package xt-oss --example api_bucket_cname_del`
//!
//! 调用DeleteCname接口删除某个存储空间(Bucket)已绑定的Cname
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/deletecname)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_cname_del.rs)
use dotenv;
use std::process;
use xt_oss::{oss, util};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);
    // 测试域名改成自己的域名
    let cname_domain = "xtoss-web.example.com";
    let result = client
        .DeleteCname(cname_domain)
        .execute()
        .await
        .unwrap_or_else(|reqwest_error| {
            println!("reqwest error: {}", reqwest_error);
            process::exit(-1);
        });
    match result {
        Ok(oss_data) => {
            println!("{:#?}", oss_data.headers())
        }
        Err(error_message) => {
            println!("{}", error_message.content())
        }
    }
    Ok(())
}
