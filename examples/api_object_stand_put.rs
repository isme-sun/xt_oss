use std::{
  env,
  fs::File,
  io::{self, Read},
  process,
};

use base64::{engine::general_purpose, Engine as _};
use crypto::{digest::Digest, md5::Md5};
use xt_oss::{
  oss::{
    self,
    entities::{ObjectACL, StorageClass},
  },
  utils,
};

// [doc](https://help.aliyun.com/zh/oss/developer-reference/include-signatures-in-the-authorization-header#section-i74-k35-5w4)
fn base64_encode(content: &str) -> String {
  let bytes = hex::decode(&content).unwrap();
  let encoded = general_purpose::STANDARD.encode(&bytes);
  encoded
}

fn file_md5(file: &str) -> Result<String, io::Error> {
  let mut file = File::open(file)?;
  let mut hasher = Md5::new();
  let mut buffer = [0; 1024];
  loop {
    let bytes_read = file.read(&mut buffer)?;
    if bytes_read == 0 {
      break;
    }
    hasher.input(&buffer[..bytes_read]);
  }
  Ok(hasher.result_str())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  dotenv::dotenv().ok();
  // 获取配置选项和 OSS 客户端
  let options = utils::options_from_env();
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
  dbg!(&target_file);
  // md5值
  let content_md5 = &base64_encode(&file_md5(&target_file)?[..]);
  dbg!(&content_md5);
  // 获得mime
  let content_type = {
    let mime = mime_guess::from_path(&target_file).first().unwrap();
    &mime.to_string()[..]
  };
  dbg!(&content_type);
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
