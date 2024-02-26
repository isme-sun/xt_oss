//! `cargo run --example aa_list_object_v2_all -q`
//!
//! 这段代码是一个示例程序，用于列举 OSS 存储桶中的所有对象，并显示每个对象的 key。
//! 它演示了如何使用 Rust 语言与 OSS 进行对象列表操作。
//!
//! 以下是代码的主要逻辑：
//!
//! 1. 导入所需的库和模块。
//! 2. 从环境变量中获取配置信息，并创建 OSS 客户端。
//! 3. 定义一个循环，用于不断获取对象列表，直到所有对象都被列举完毕。
//! 4. 在循环中，使用 ListObjectsV2 操作来获取对象列表，每次最多获取 5 个对象。
//! 5. 如果成功获取到对象列表，就逐个打印出对象的 key，并统计文件数量和总大小。
//! 6. 如果遇到错误，就打印错误信息并退出循环。
//! 7. 最后，打印出文件数量和总大小。
//!
//! -- ChatGPT3.5 解读
use std::process;

use dotenv;
use xt_oss::prelude::*;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);

    let mut token: Option<String> = None;
    let mut count = 0u32;
    let mut size = 0u64;
    let mut page = 1;
    loop {
        match client
            .ListObjectsV2()
            .with_max_keys(5)
            // .with_prefix("txt")
            // .with_encoding_type("url")
            .with_continuation_token(token.as_deref())
            .execute()
            .await
            .unwrap_or_else(|error| {
                println!("reqwest error: {}", error);
                process::exit(-1);
            }) {
            Ok(data) => {
                let objects = data.content();
                if objects.key_count == Some(0) {
                    println!("not object");
                } else {
                    token = objects.next_continuation_token.clone();
                    println!("-- PAGE {} --", page);
                    for object in objects.contents.unwrap() {
                        size = size + object.size as u64;
                        println!(" - {}", object.key);
                    }
                }
                count = count + objects.key_count.unwrap();
                page = page + 1;
            }
            Err(message) => {
                println!("oss error {}:", message.url());
                println!("oss error {:#?}:", message.content());
                break;
            }
        }
        if token.is_none() {
            break;
        }
    }

    println!("{}", "-".repeat(60));
    println!("file count: {}", count);
    println!("size: {:.2}MB", (size as f64) / 1024f64 / 1024f64);

    Ok(())
}
