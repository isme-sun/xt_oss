//! cargo run --example api_bucket_list_cname -q
//!
//! 调用ListCname接口用于查询某个存储空间(Bucket)下绑定的所有的自定义域名(Cname)列表
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/listcname)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_cname_list.rs)
use dotenv;
use std::process;
use xt_oss::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);
    let result = client.ListCname().execute().await.unwrap_or_else(|error| {
        // 请求错误
        println!("reqwest error: {}", &error);
        if let Some(status) = error.status() {
            println!("is_client_error: {}", status.is_client_error());
            println!("is_informational: {}", status.is_informational());
            println!("is_redirection: {}", status.is_redirection());
            println!("is_server_error: {}", status.is_server_error());
        }
        process::exit(-1);
    });

    match result {
        Ok(data) => {
            println!("request id: {}", data.request_id());
            println!("{:#?}", data.headers());
            let content = data.content();
            println!("{:#?}", &content);
            println!("{}", serde_json::to_string_pretty(&content).unwrap())
        }
        Err(error) => {
            println!("request id: {}", error.request_id());
            println!("{:#?}", error.headers());
            println!("{:#?}", error.content())
        }
    }
    Ok(())
}
