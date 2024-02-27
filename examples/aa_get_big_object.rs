//! `cargo run --example aa_get_big_object -q`
//!
//! 这段代码是一个示例程序，用于从 OSS 下载大文件，并将其保存到本地。它演示了如何使用 Rust 语言与 OSS 
//! 进行文件的分段下载，以及如何将下载的文件分段保存到本地。
//!
//! > 以下是代码的主要逻辑：
//!
//! 1. 导入所需的库和模块。
//! 2. 从环境变量中获取配置信息，并创建 OSS 客户端。
//! 3. 定义要下载的目标文件对象路径和本地存储位置。
//! 4. 获取目标文件的总大小，并计算出文件的分段数。
//! 5. 使用分段下载的方式，依次下载每个分段的文件内容，并将其保存到 BytesMut 类型的缓冲区中。
//! 6. 将所有分段的文件内容合并到一起，并保存到本地文件中。
//!
//! -- ChatGPT3.5 解读
use dotenv;
use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
    process,
};
use xt_oss::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    println!("\n# 下载大文件 example\n");
    let options = util::options_from_env();
    let client = oss::Client::new(options);
    // 下载目标文件
    let object = "images/JPGImage_30mbmb.jpg";
    // 下载后本地存储位置
    let down_dir = {
        let base_dir = match dirs::home_dir() {
            Some(path) => path,
            None => std::env::temp_dir(),
        };
        let mut down_dir = PathBuf::from(base_dir);
        down_dir.push("xtoss");
        down_dir.push("down");
        down_dir
    };
    let save_path = down_dir.clone().join(object);
    println!(" - save location: {}", save_path.display());

    // 分段size
    let chunk_size = 1024 * 1024;
    let size = if let Ok(data) = client.GetObjectMeta(object).execute().await? {
        // 从header获取目标文件的size
        data.content_length()
    } else {
        None
    };
    assert!(size.is_some(), "Failed to retrieve file size, perhaps the file does not exist.");
    println!(
        " - total file size: {:.2} MB",
        size.unwrap() as f64 / chunk_size as f64
    );
    let byte_range_list = ByteRange::chunk(size.unwrap(), chunk_size);
    let byte_range_len = byte_range_list.len();

    let mut bytes = oss::BytesMut::new();

    for (index, byte_range) in byte_range_list.iter().enumerate() {
        match client
            .GetObject(object)
            .with_range(byte_range.clone())
            .execute()
            .await
            .unwrap_or_else(|error| {
                println!("reqwest error: {}", error);
                process::exit(-1);
            }) {
            Ok(data) => {
                let rate = format!(
                    " - complete {:5.2}%",
                    ((index + 1) as f64 / byte_range_len as f64) * 100f64
                );
                if index == 0 {
                    print!("{rate}");
                    io::stdout().flush()?;
                } else {
                    print!("\r{rate}");
                    io::stdout().flush()?;
                }
                bytes.extend(data.content())
            }
            Err(message) => println!("oss error: {}", message.content()),
        }
    }

    println!();
    if let Some(dirname) = save_path.parent() {
        if !dirname.is_dir() {
            fs::create_dir_all(dirname)?;
        }
        let mut file = fs::File::create(save_path)?;
        file.write_all(&bytes)?;
        println!(" - save sucecess!");
    }

    Ok(())
}
