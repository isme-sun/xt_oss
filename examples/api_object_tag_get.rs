//! `cargo run --example api_object_tag_get -q`
//!
//! 调用GetObjectTagging接口获取对象`Object`的标签`Tagging`信息。
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/getobjecttagging)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_object_tag_get.rs)
use dotenv;
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
    {
        Ok(Ok(data)) => {
            // data:ApiData<Tagging>
            println!("{}", data.request_id());
            println!("{:#?}", data.headers());
            println!("{:#?}", data.content());
        }
        Ok(Err(message)) => {
            // message: ApiData<ErrorMessage>
            println!("{}", message.request_id());
            println!("{:#?}", message.headers());
            println!("{:#?}", message.content());
        }
        Err(reqwest_error) => println!("{}", reqwest_error),
    }
    Ok(())
}
