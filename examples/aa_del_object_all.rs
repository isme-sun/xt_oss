//! `cargo run --example aa_del_object_all -q`
//!
//! 这段代码是一个示例程序，用于删除指定存储桶中的所有对象。它演示了如何使用 Rust 语言与 OSS 进行对象管理操作，
//! 并使用异步编程模型来提高效率。
//!
//! 以下是代码的主要逻辑：
//! 
//! 1. 导入所需的库和模块。
//! 2. 定义了一个异步函数 del_all()，用于删除指定存储桶中的所有对象。
//! 3. 在 main() 函数中，先打印警告信息，然后提示用户是否继续操作。
//! 4. 用户输入 Y 表示继续操作，调用 del_all() 函数进行对象删除操作。
//! 5. 用户输入 n 表示取消操作，程序结束。
//! 6. 用户输入其他值时，提示用户输入 Y 或 n。
//! 7. del_all() 函数中，使用循环列出存储桶中的所有对象，并发起删除操作。删除操作使用异步任务进行并行处理。
//! 8. 每次删除操作完成后，输出相应的结果信息。
//!
//! -- ChatGPT3.5 解读
use dotenv;
use futures::future::join_all;
use std::io::{self, Write};
use std::{process, sync::Arc};
use xt_oss::prelude::*;

async fn del_all() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = Arc::new(oss::Client::new(options));
    loop {
        match client
            .ListObjectsV2()
            .with_max_keys(5)
            .execute()
            .await
            .unwrap_or_else(|error| {
                println!("reqwest error: {}", error);
                process::exit(-1);
            }) {
            Ok(data) => {
                let data = data.content();
                if let Some(objects) = data.contents {
                    let tasks = objects.into_iter().map(|object| {
                        let client = Arc::clone(&client);
                        async move {
                            let result = client.DeleteObject(object.key.as_str()).execute().await;
                            match result {
                                Ok(Ok(_)) => println!("delete object key: {}", object.key),
                                Ok(Err(message)) => println!("{}", message.content()),
                                Err(error) => println!("reqwest error: {}", error),
                            }
                        }
                    });
                    join_all(tasks).await;
                } else {
                    println!("bucket file clean");
                    return Ok(());
                }
            }
            Err(message) => {
                println!("oss error: {}", message.content())
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    println!("警告：后续操作可能存在危险！");
    loop {
        print!("是否继续操作?(Y/n):");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().to_uppercase().as_str() {
            "Y" => {
                println!("继续运行...");
                del_all().await?;
                break;
            }
            "n" => {
                println!("操作已取消。");
                break;
            }
            _ => {
                println!("无效的输入，请输入 Y 或 n。");
            }
        }
    }
    Ok(())
}
