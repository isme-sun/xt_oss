//! `cargo run --example aa_create_buckets -q`
//!
//! 这段代码是一个示例程序，用于创建多个存储桶（Bucket）并将其部署到指定的 OSS 区域（Region）。代码的逻辑是循环创建存储桶，并使用并发的方式发送创建存储桶的请求，以提高效率。
//!
//! 以下是代码的主要逻辑：
//!
//! 导入所需的库和模块。
//! 从环境变量中获取配置信息，并创建 OSS 客户端。
//! 定义存储桶名称列表，并使用 map 方法生成包含存储桶名称的向量。
//! 使用 join_all 函数和异步 map 方法并发执行创建存储桶的任务。
//! 对于每个存储桶，构建创建存储桶的请求，并使用 await 关键字等待请求完成。
//! 处理创建存储桶的结果，如果成功则打印成功消息，如果失败则打印错误信息。
//!
//! -- ChatGPT3.5 解读
use dotenv;
use futures::future::join_all;
use std::sync::Arc;
use xt_oss::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = Arc::new(oss::Client::new(options));
    let region = "oss-cn-shanghai".to_string();

    let buckets = (10..20)
        .map(|i| format!("xtoss-ex{}", i))
        .collect::<Vec<String>>();

    let futures = buckets.into_iter().map(|bucket| {
        let client = Arc::clone(&client);
        let region = region.clone();
        async move {
            match client
                .PutBucket()
                .with_bucket(&bucket)
                .with_region(&region)
                .execute()
                .await
            {
                Ok(Ok(_)) => println!("create {}@{} is success", &bucket, &region),
                Ok(Err(error)) => println!("{:#?}", error.content()),
                Err(error) => println!("reqwest error: {}", error),
            }
        }
    });

    join_all(futures).await;
    Ok(())
}
