//! `cargo run --example api_object_stand_put -q`
//!
//! DeleteMultipleObjects接口用于删除同一个存储空间`Bucket`中的多个文件`Object`
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/deletemultipleobjects)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_object_stand_mutil_del.rs)
use std::{env, fs::File, io::Read, process};

use xt_oss::{
    oss::entities::{
        callback::{self, builder::CallbackBuilder, CallbackBody, CallbackBodyItem},
        ObjectACL, StorageClass,
    },
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
        // ["examples", "samples", "zip", "ZIPFile_10mbmb.zip"]
        ["assets", "samples","images", "JPGImage_2mbmb.jpg"].iter().for_each(|e| {
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

    let callback_body = CallbackBody::new()
        .with_items(
            [
                CallbackBodyItem::Bucket,
                CallbackBodyItem::ClientIp,
                CallbackBodyItem::ContentMD5,
                CallbackBodyItem::Crc64,
                CallbackBodyItem::Object,
                CallbackBodyItem::Operation,
                CallbackBodyItem::ReqId,
                CallbackBodyItem::VpcId,
                CallbackBodyItem::Etag,
                CallbackBodyItem::Size,
                CallbackBodyItem::MimeType,
                CallbackBodyItem::ImageInfoHeight,
                CallbackBodyItem::ImageInfoFormat,
                CallbackBodyItem::ImageInfoWidth,
            ]
            .to_vec(),
        )
        .with_custom_items([("author", "李白"), ("published", "now")].to_vec());

    let callback = CallbackBuilder::new()
        .with_url(vec!["https://dev-service.xuetube.com/system/xtoss/cb"])
        .with_body(callback_body)
        .with_body_type(callback::CallbackBodyType::FormUrlEncoded)
        .build();

    // 上传文件到 OSS
    let resp = client
        .PutObject("tmp/tmp.jpg")
        .with_object_acl(ObjectACL::PublicRead)
        .with_content_type(content_type)
        .with_storage_class(StorageClass::Standard)
        .with_content(content)
        .with_content_md5(&content_md5)
        .with_oss_tagging(vec![("k100", "v100"), ("k200", "v200")])
        .with_timeout(30)
        .with_callback(callback)
        .execute()
        .await
        .unwrap_or_else(|error| {
            eprint!("reqwest error: {}", error);
            process::exit(-1);
        });

    // 处理响应结果
    match resp {
        Ok(data) => {
            println!("{:#?}", data.headers());
            let content = data.content();
            if !content.is_empty() {
                let txt = String::from_utf8_lossy(&content);
                println!("===json===");
                println!("{}", txt);
                println!("===json===");
                // let v: Res = serde_json::from_str(&txt).unwrap();
            } else {
                println!("no content");
            }
        }
        Err(message) => println!("{:#?}", message),
    }

    Ok(())
}
