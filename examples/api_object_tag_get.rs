//! `cargo run --example api_object_tag_get -q`
//!
//! 调用GetObjectTagging接口获取对象`Object`的标签`Tagging`信息。
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/getobjecttagging)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_object_tag_get.rs)
use dotenv;
use std::process;
use xt_oss::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);

    match client
        .GetObjectTagging("excel/Spreadsheet-1000-rows.xls")
        .execute()
        .await
        .unwrap_or_else(|error| {
            println!("{}", error);
            process::exit(-1);
        }) {
        Ok(data) => {
            println!("{}", serde_json::to_string_pretty(&data.content())?);
        }
        Err(message) => {
            println!("{:#?}", message.content())
        }
    }

    Ok(())
}
