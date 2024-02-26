//! `cargo run --example aa_sync_sample_down -q`
//!
//! 这段代码是一个示例程序，用于从 OSS（对象存储服务）中下载文件到本地目录。代码的逻辑是不断地向 OSS 发送请求，
//! 获取文件列表，并逐个下载文件到本地目录。
//!
//! 以下是代码的主要逻辑：
//! 1. 导入所需的库和模块。
//! 2. 从环境变量中获取配置信息，并创建 OSS 客户端。
//! 3. 定义下载目录，并创建该目录（如果不存在）。
//! 4. 使用循环从 OSS 中获取文件列表，并逐个下载文件。
//! 5. 对于每个文件，构建下载请求并执行下载操作，将文件保存到本地目录中。
//! 6. 处理下载过程中的错误，如果有错误则打印错误信息。
//!
//! 代码中使用了 ListObjectsV2 方法获取文件列表，然后使用 GetObject 方法下载文件。对于每个文件，都将其保存到指定的本地目录中，并打印下载成功的消息。如果下载过程中发生了错误，则会打印错误信息并终止程序的执行。
//!
//! -- ChatGPT3.5 解读
use dotenv;
use std::{env, fs, io::Write, path::PathBuf};
use xt_oss::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);

    let down_dir = {
        let base_dir = env::var("HOME").unwrap_or(env::temp_dir().display().to_string());
        let mut down_dir = PathBuf::from(base_dir);
        down_dir.push("xtoss");
        down_dir.push("samples");
        down_dir
    };

    fs::create_dir_all(&down_dir)?;
    println!("down file to {}", down_dir.display());

    let mut token: Option<String> = None;
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
                std::process::exit(-1);
            }) {
            Ok(data) => {
                let objects = data.content();
                if let Some(0) = objects.key_count {
                    println!("not object");
                } else {
                    token = objects.next_continuation_token.clone();
                    for object in objects.contents.unwrap() {
                        // 下载文件
                        match client.GetObject(&object.key).execute().await {
                            Ok(Ok(data)) => {
                                let target_file = down_dir.clone().join(&object.key);
                                if let Some(dirname) = target_file.parent() {
                                    if !dirname.is_dir() {
                                        fs::create_dir_all(dirname)?;
                                    }
                                }
                                let mut file = fs::File::create(target_file)?;
                                // 写入内容
                                file.write_all(&data.content())?;
                                println!("down file: {}", &object.key);
                            }
                            Ok(Err(message)) => println!("oss error: {}", message.content()),
                            Err(error) => println!("reqwest oss: {}", error),
                        }
                    }
                }
            }
            Err(message) => {
                println!("oss error {:#?}:", message.content());
                break;
            }
        }
        if token.is_none() {
            break;
        }
    }

    Ok(())
}
