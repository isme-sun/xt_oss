use std::{env, fs::File, io::Read, process};

use xt_oss::{
    oss::entities::{ObjectACL, StorageClass},
    prelude::*,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    // 获取配置选项和 OSS 客户端
    let options = util::options_from_env();
    let client = oss::Client::new(options);
    // 目标文件
    let target_file = {
        let mut target_file = env::current_dir()?;
        ["examples", "samples", "zip", "ZIPFile_10mbmb.zip"]
            .iter()
            .for_each(|e| {
                target_file.push(e);
            });
        target_file.display().to_string()
    };
    // md5值
    let content_md5 = util::oss_file_md5(&target_file)?;
    // 获得mime
    let content_type = {
        let mime = mime_guess::from_path(&target_file).first().unwrap();
        &mime.to_string()[..]
    };
    // 获取内容
    let content = {
        let mut current_file = File::open(&target_file)?;
        let mut content = vec![];
        current_file.read_to_end(&mut content)?;
        oss::Bytes::from(content)
    };

    // 上传文件到 OSS
    let resp = client
        .PutObject("tmp/test.zip")
        .with_object_acl(ObjectACL::PublicRead)
        .with_content_type(content_type)
        .with_storage_class(StorageClass::Standard)
        .with_content(content)
        .with_content_md5(&content_md5)
        .with_oss_tagging(vec![("k100", "v100"), ("k200", "v200")])
        // .with_timeout(120)
        .execute()
        .await
        .unwrap_or_else(|error| {
            eprint!("reqwest error: {}", error);
            process::exit(-1);
        });

    // 处理响应结果
    match resp {
        Ok(data) => println!("{:#?}", data.headers()),
        Err(message) => println!("{:#?}", message.content()),
    }

    Ok(())
}
