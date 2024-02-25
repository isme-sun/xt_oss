//! `cargo run --example api_service_list_buckets -q`
//!
//! 调用ListBuckets（GetService）接口列举请求者拥有的所有存储空间（Bucket）。
//! 您还可以通过设置prefix、marker或者max-keys参数列举满足指定条件的存储空间。
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/listbuckets)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_service_list_buckets.rs)
use dotenv;
use std::process;
use xt_oss::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);

    let result = client
        .ListBuckets()
        // .with_marker("marker")
        // .with_max_keys(5)
        .with_prefix("xtoss")
        // .with_resource_group_id("group_id")
        .execute()
        .await
        .unwrap_or_else(|error| {
            println!("reqwest error: {}", error);
            process::exit(-1);
        });

    match result {
        Ok(data) => {
            let all_buckets = data.content();
            // println!("{:#?}", all_buckets);
            if let Some(buckets) = &all_buckets.buckets.bucket {
                for bucket in buckets {
                    println!("{}", bucket.name);
                    println!("{}", "-".repeat(42));
                    println!(" - storage_class: {}", bucket.storage_class);
                    println!(" - creation_date: {}", bucket.creation_date);
                    println!(" -      location: {}", bucket.location);
                    println!(" -       comment: {}",
                        bucket.comment.as_deref().unwrap_or_default()
                    );
                    println!()
                }
            } else {
                // println!("{:#?}", all_buckets);
                println!("no buckets");
            }
        }
        Err(message) => println!("oss error: {}", message.content()),
    }
    Ok(())
}
