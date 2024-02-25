//! cargo run --example api_bucket_acl_get -q
//!   
//! GetBucketAcl接口用于获取某个存储空间`Bucket`的访问权限`ACL`。
//! 只有Bucket的拥有者才能获取Bucket的访问权限。
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/getbucketacl)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_acl_get.rs)
use dotenv;
use serde_json;
use std::process;
use xt_oss::prelude::*;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);

    match client
        .GetBucketAcl()
        .execute()
        .await
        .unwrap_or_else(|reqwest_error| {
            println!("reqwest error: {}", reqwest_error);
            process::exit(-1);
        }) {
        Ok(oss_data) => {
            println!(
                "{}",
                serde_json::to_string_pretty(&oss_data.content()).unwrap()
            );
        }
        Err(error_message) => {
            println!("{:#?}", error_message.content());
        }
    }
}
