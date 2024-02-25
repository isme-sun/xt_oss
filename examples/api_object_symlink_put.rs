//! `cargo run --example api_object_symlink_put -q`
//!
//! 调用PutSymlink接口用于为OSS的目标文件`TargetObject`创建软链接
//! `Symlink`,您可以通过该软链接访问TargetObject。
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/putsymlink)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_object_symlink_put.rs)
use std::process;

use xt_oss::{
    oss::entities::{ObjectACL, StorageClass},
    prelude::*,
};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);

    let content_disposition = http::ContentDisposition::ATTACHMENT(Some("测试.xml".to_string()));
    match client
        .PutSymlink("tmp/test.xls")
        .with_symlink_target("excel/Spreadsheet-1000-rows.xls")
        // .with_forbid_overwrite(false)
        .with_content_type("application/vnd.ms-excel")
        .with_object_acl(ObjectACL::Default)
        .with_storage_class(StorageClass::Archive)
        .with_content_disposition(content_disposition)
        .with_oss_meta("name1", "sjy")
        .with_oss_meta("name2", "sun")
        .execute()
        .await
        .unwrap_or_else(|error| {
            println!("{}", error);
            process::exit(-1);
        }) {
        Ok(oss_data) => {
            println!("{:#?}", oss_data.headers())
        }
        Err(error_message) => {
            println!("{:#?}", error_message.content())
        }
    }
}
