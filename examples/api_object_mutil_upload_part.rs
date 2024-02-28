//! `cargo run --package xt-oss --example api_object_mutil_upload_part -q`
//!
//! 初始化一个MultipartUpload后,调用UploadPart接口根据指定的Object名和uploadId来分块`Part`上传数据。
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/uploadpart)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_object_mutil_upload_part.rs)
//!
//! 这个例子展示了如何使用 xt_oss 库进行分块上传大文件到阿里云 OSS。主要的逻辑如下:
//!
//! 1. 初始化一个 Multipart Upload,获取上传会话的 UploadId。
//! 2. 将文件分成固定大小的块`chunk`。
//! 3. 对每个块执行上传操作，使用 UploadPart 接口上传分块数据，每个块对应一个 PartNumber。
//! 4. 完成分块上传，调用 CompleteMultipartUpload 接口完成整个上传过程。
//!
//! 这个例子中，通过迭代器将文件分块，并在每个块上执行上传操作，最后完成整个文件的上传。在上传过程中，
//! 可以根据需要处理上传成功或失败的情况，并进行相应的处理。
//!
//! -- ChatGPT3.5 解读
use dotenv;
use std::io::{Seek, SeekFrom};
use std::{env, fs, io::Read, process};
use xt_oss::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);

    let target_file = {
        let mut current_dir = env::current_dir()?;
        "assets/samples/images/JPGImage_30mbmb.jpg"
            .split("/")
            .for_each(|e| current_dir.push(e));
        current_dir
    };

    let file_size = fs::metadata(&target_file)?.len();
    let chunk_size = 1024 * 1024;
    let object = "tmp/temp.jpg";
    let mut file = fs::File::open(&target_file)?;

    let upload_id = client
        .InitiateMultipartUpload(object)
        .execute()
        .await
        .unwrap_or_else(|error| {
            eprintln!("reqwest error: {}", error);
            process::exit(-1);
        })
        .unwrap_or_else(|message| {
            eprintln!("oss error: {}", message.content());
            process::exit(-1);
        })
        .content()
        .upload_id;

    let chunks = ByteRange::chunk(file_size, chunk_size);
    let file_chunks = chunks.iter().map(|range| {
        let (seek, length) = (range.start(), range.amount() as u64);
        let mut buffer = vec![0; length as usize];
        let _ = file.seek(SeekFrom::Start(seek));
        let _ = file.read_exact(&mut buffer);
        oss::Bytes::from(buffer)
    });

    for (i, content) in file_chunks.enumerate() {
        let part_number = i + 1;
        let result = client
            .UploadPart(object)
            .with_part_number(part_number as u32)
            .with_upload_id(&upload_id)
            .with_content(content)
            .execute()
            .await
            .unwrap()
            .unwrap();
        println!("part {:#?}", result.headers())
    }

    match client
        .CompleteMultipartUpload(object)
        .with_upload_id(&upload_id)
        .with_encoding_type("url")
        .with_forbid_overwrite(false)
        .execute()
        .await
    {
        Ok(Ok(data)) => println!("{:#?}", data.content()),
        Ok(Err(message)) => println!("{:#?}", message.content()),
        Err(error) => println!("{}", error),
    }

    Ok(())
}
