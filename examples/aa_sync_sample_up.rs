//! `cargo run --example aa_sync_sample_up -q`
//! 上传测试文件到bucket
//!
//! ChatGPT3.5 解读
//! 这段代码是一个示例程序，用于将示例文件上传到 OSS（对象存储服务）中。代码的逻辑是遍历指定目录下的所有文件，将它们逐个上传到 OSS 中。
//!
//! 以下是代码的主要逻辑：
//!
//! 1. 导入所需的库和模块。
//! 2. 从环境变量中获取配置信息，并创建 OSS 客户端。
//! 3. 获取示例文件所在的目录路径。
//! 4. 遍历示例文件目录下的所有文件，并过滤出文件。
//! 5. 为每个文件构建上传请求，并执行上传操作。
//! 6. 处理上传结果，如果成功则打印上传成功的消息，如果失败则打印错误信息。
//! 代码中使用了 `WalkDir` 库来遍历目录，并使用 `mime_guess` 库来推断文件的 MIME 类型。在上传文件时，使用了各种 
//! OSS 参数设置，如禁止覆盖、设置内容编码、缓存控制、内容类型等。此外，还设置了对象的元数据、加密方式、过期时间以及标签。
//! 整体来说，这段代码是一个完整的示例，展示了如何使用 Rust 语言与 OSS 进行文件上传操作，并对上传过程中的错误进行了简单
//! 的处理和打印。
//!
//! -- ChatGPT3.5 解读
use chrono::Utc;
use dotenv;
use std::{env, fs::File, io::Read};
use walkdir::{DirEntry, WalkDir};
use xt_oss::{
    oss::{
        entities::ServerSideEncryption,
        http::{CacheControl, ContentDisposition, ContentEncoding},
    },
    prelude::*,
    util::utc_to_gmt,
};

fn only_file(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map_or(false, |s| entry.depth() == 0 || !s.starts_with("."))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let option = util::options_from_env();
    let client = oss::Client::new(option);

    let basedir = env::current_dir()?;
    let mut sampledir = basedir.clone();
    sampledir.push("examples");
    sampledir.push("samples");

    if !sampledir.is_dir() {
        panic!("not exists")
    }

    for entry in WalkDir::new(&sampledir)
        .into_iter()
        .filter_entry(|e| only_file(e))
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let file_path = entry.path().to_string_lossy();
        let object = file_path
            .chars()
            .skip(sampledir.to_string_lossy().len() + 1)
            .collect::<String>(); // Extract object path

        let mime = match mime_guess::from_path(&object).first() {
            Some(mime) => mime.to_string(),
            None => oss::DEFAULT_CONTENT_TYPE.to_string(),
        };
        let mut current_file = File::open(&file_path.to_string())?;
        let mut content = vec![];
        current_file.read_to_end(&mut content)?;
        let content = oss::Bytes::from(content);

        println!("upload file {}", &object);
        match client
            .PutObject(&object)
            .with_forbid_overwrite(true)
            .with_content_encoding(ContentEncoding::IDENTITY)
            .with_cache_control(CacheControl::NoCache)
            .with_content_disposition(ContentDisposition::ATTACHMENT(Some(
                "myfile.tmp".to_string(),
            )))
            .with_content_type(&mime)
            // .with_content_language("zh-CN")
            .with_content(content)
            .with_oss_meta(vec![
                ("upload-at", &Utc::now().timestamp().to_string()),
                ("upload-by", "xtoss"),
            ])
            .with_encryption(ServerSideEncryption::AES256)
            .with_expires(&utc_to_gmt(Utc::now()))
            .with_oss_tagging(vec![("tag1", "value1"), ("tag2", "value1")])
            .execute()
            .await
        {
            Ok(Ok(_)) => (),
            Ok(Err(message)) => println!("oss error: {}", message.content()),
            Err(error) => println!("reqwest error: {}", error),
        }
    }

    Ok(())
}
