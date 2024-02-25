//! ` cargo run --example api_object_stand_copy -q`
//!
//! 调用CopyObject接口拷贝同一地域下相同或不同存储空间`Bucket`之间的文件`Object`
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/copyobject)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_object_stand_copy.rs)
use dotenv;
use std::process;
use xt_oss::{oss::entities::object::TaggingDirective, prelude::*};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);

    let tags = [("k1", "v1"), ("k2", "v2"), ("k3", "v3")].to_vec();

    match client
        .CopyObject("tmp/tmp/copy_test_index.html")
        .with_copy_source("/xtoss-ex9/index.html")
        .with_oss_tagging(tags)
        .with_tagging_directive(TaggingDirective::REPLACE)
        .execute()
        .await
        .unwrap_or_else(|error| {
            println!("reqwest error: {}", error);
            process::exit(-1);
        }) {
        Ok(oss_data) => {
            println!("{:#?}", oss_data.content());
        }
        Err(error_message) => {
            println!("{:#?}", error_message.content())
        }
    }
    Ok(())
}
