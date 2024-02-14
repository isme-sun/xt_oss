use std::{env, fs, io::Read, path::PathBuf, process};

use xt_oss::{
    oss::{
        self,
        entities::{ObjectACL, StorageClass},
        Bytes,
    },
    utils,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    // 获取配置选项和 OSS 客户端
    let options = utils::options_from_env();
    let client = oss::Client::new(options);

    // 设置本地文件路径
    let basedir = env::current_dir()?;
    let mut sampledir = basedir.clone();
    sampledir.push("examples");
    sampledir.push("samples");

    // 检查示例文件夹是否存在
    if !sampledir.is_dir() {
        panic!("Sample directory does not exist");
    }

    // 设置本地文件路径和文件名
    let mut local_file_path = PathBuf::from(&sampledir);
    local_file_path.push("index.html");

    // 读取本地文件内容
    let mut content = Vec::new();
    let mut file = fs::File::open(&local_file_path)?;
    file.read_to_end(&mut content)?;
    let content = Bytes::from(content);

    // 上传文件到 OSS
    let resp = client
        .PutObject("index.html")
        .with_object_acl(ObjectACL::PublicRead)
        .with_storage_class(StorageClass::Standard)
        .with_content(content)
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
