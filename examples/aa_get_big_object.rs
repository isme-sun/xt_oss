//! `cargo run --example aa_get_big_object -q`
#![deny(warnings)]

use bytes::BytesMut;
use dotenv;
use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
    process,
};
use xt_oss::{
    oss,
    utils::{self, ByteRange},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    println!("\n# 下载大文件 example\n");
    let options = utils::options_from_env();
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
    assert!(size.is_some());
    println!(" - total file size: {:.2} MB", size.unwrap() as f64 / chunk_size as f64);
    let byte_range_list = ByteRange::chunk(size.unwrap(), chunk_size);
    let byte_range_len = byte_range_list.len();

    let mut bytes = BytesMut::new();

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
