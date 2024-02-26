//! `cargo run --example api_object_acl_put -q`
//! 
//! 调用PutObjectACL接口修改文件`Object`的访问权限`ACL`。
//! 此操作只有Bucket Owner有权限执行,且需对Object有读写权限。
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/putobjectacl)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_object_acl_put.rs)
use dotenv;
use std::process;
use xt_oss::{oss::entities::ObjectACL, prelude::*};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);

    match client
        .PutObjectACL("excel/Spreadsheet-1000-rows.xls")
        .with_acl(ObjectACL::PublicReadWrite)
        .with_version_id("CAEQmgEYgYDA9I_smO0YIiBhOGJmMTczNzY0ZmM0NTE1YTA5MDJlOWE1YmI1ZTZlNQ--")
        .execute()
        .await
        .unwrap_or_else(|error| {
            println!("{}", error);
            process::exit(-1);
        }) {
        Ok(data) => {
            println!("{:#?}", data.headers())
        }
        Err(message) => {
            println!("{:#?}", message.content())
        }
    }
    Ok(())
}
