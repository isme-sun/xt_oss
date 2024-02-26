//! `cargo run --example aa_delete_all_buckets -q`
//!
//! 删除`xtoss-`开头的测试`bucket``

//! 这段代码是一个示例程序，用于删除所有以 xtoss- 开头的测试存储桶（bucket）。它演示了如何使用 
//! Rust 语言与 OSS 进行存储桶管理操作。
//!
//! 以下是代码的主要逻辑：
//!
//! 1. 导入所需的库和模块。
//! 2. 从环境变量中获取配置信息，并创建 OSS 客户端。
//! 3. 使用 ListBuckets 操作列出所有以 xtoss- 开头的存储桶。
//! 4. 遍历返回的存储桶列表，逐个删除每个存储桶。
//! 5. 如果成功删除存储桶，则打印出删除成功的消息，否则打印出错误信息。
//!
//! -- ChatGPT3.5 解读
use xt_oss::{oss::entities::bucket::ListAllMyBucketsResult, prelude::*};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);

    // 列出所有以xtoss-开头的bucket
    let all_buckets: ListAllMyBucketsResult = client
        .ListBuckets()
        .with_prefix("xtoss-")
        .execute()
        .await?
        .unwrap()
        .content();

    if let Some(bucktes) = all_buckets.buckets.bucket {
        for bucket in bucktes {
            let result = client
                .DeleteBucket()
                .with_bucket(&bucket.name)
                .with_region(&bucket.location)
                .execute()
                .await
                .unwrap_or_else(|error| {
                    println!("reqwest error: {}", error);
                    std::process::exit(-1);
                });
            match result {
                Ok(_) => println!("delete {} {}", &bucket.location, &bucket.name),
                Err(error) => println!("{}", error.content()),
            }
        }
    } else {
        println!("no match bucket!");
    }
    Ok(())
}
