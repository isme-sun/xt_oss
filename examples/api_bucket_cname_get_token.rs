//! `cargo run --example api_bucket_cname_get_token -q`
//!
//! 调用GetCnameToken接口获取已创建的CnameToken
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/getcnametoken)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_cname_get_token.rs)
use dotenv;
use std::process;
use xt_oss::prelude::*;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);
    let result = client
        .GetCnameToken("xtoss-ex1.xuetube.com")
        .execute()
        .await
        .unwrap_or_else(|reqwest_error| {
            println!("reqwest error: {}", reqwest_error);
            process::exit(-1);
        });

    match result {
        Ok(oss_data) => {
            println!("{:#?}", oss_data.content())
        }
        Err(error_message) => {
            println!("{}", error_message.content())
        }
    }
}
