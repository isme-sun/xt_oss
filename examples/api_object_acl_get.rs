//! `cargo run --example api_object_acl_get -q`
//! 
//! 调用GetObjectACL接口获取存储空间`Bucket`下某个文件`Object`的访问权限`ACL`。
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/getobjectacl)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_object_acl_get.rs)
use std::process;

use xt_oss::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);

    match client
        .GetObjectACL("excel/Spreadsheet-1000-rows.xls")
        // .with_version_id("CAEQmgEYgYDA9I_smO0YIiBhOGJmMTczNzY0ZmM0NTE1YTA5MDJlOWE1YmI1ZTZlNQ--")
        .execute()
        .await
        .unwrap_or_else(|error| {
            println!("{}", error);
            process::exit(-1);
        }) {
        Ok(oss_data) => {
            println!("{:#?}", oss_data.content())
        }
        Err(error_message) => {
            println!("{:#?}", error_message.content())
        }
    }
    Ok(())
}
