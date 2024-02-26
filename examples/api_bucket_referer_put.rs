//!  `cargo run --package xt-oss --example api_bucket_referer_put`
//!
//! 调用PutBucketReferer接口设置存储空间`Bucket`级别的Referer访问白名单以及黑名单
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/putbucketreferer)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_referer_put.rs)
use dotenv;
use std::process;
use xt_oss::{oss::entities::referer::builder::RefererConfigurationBuilder, prelude::*};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);

    let referer_list: Vec<&str> = vec!["https://www.xuetube.com"];
    let referer_blacklist: Vec<&str> = vec!["https://dev.xuetube.com"];

    let result = client
        .PutBucketReferer()
        .with_config(
            RefererConfigurationBuilder::new()
                .with_allow_empty_referer(false)
                .with_allow_truncate_query_string(false)
                .with_truncate_path(false)
                .with_referer_list(referer_list)
                .with_referer_blacklist(referer_blacklist)
                .build(),
        )
        .execute()
        .await
        .unwrap_or_else(|reqwest_error| {
            println!("reqwest error: {}", reqwest_error);
            process::exit(-1);
        });

    match result {
        Ok(oss_data) => {
            println!("{:#?}", oss_data.headers());
        }
        Err(oss_error) => {
            println!("oss error:{:#?}", oss_error.content())
        }
    }
    Ok(())
}
