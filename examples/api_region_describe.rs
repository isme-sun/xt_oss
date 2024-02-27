//! `cargo run --example api_region_describe -q`
//!
//! 调用DescribeRegions接口查询所有支持地域或者指定地域对应的Endpoint信息，
//! 包括外网Endpoint、内网Endpoint和传输加速Endpoint。
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/describeregions)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_describe_regions.rs)
use dotenv;
use std::process;
use xt_oss::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    // let options = oss::Options::new()
    //     .with_access_key_id("-- your access_key_id --")
    //     .with_access_key_secret("-- your access_key_secret --");

    let client = oss::Client::new(options);

    match client
        .DescribeRegions()
        // .with_region("oss-us-east-1")
        .execute()
        .await
        // 处理可能的reqwest错误
        .unwrap_or_else(|reqwest_error| {
            println!("reqweset error: {}", reqwest_error);
            process::exit(-1);
        }) {
        // 请求正常返回结果
        Ok(oss_data) => {
            oss_data.content().region_info.iter().for_each(|entry| {
                println!("{:>20} | {}", entry.region, entry.internet_endpoint);
            });
        }
        // 请求正常，返回oss错误消息
        Err(error_message) => {
            // let message = error_message.content();
            println!("request id: {}", &error_message.request_id());
            println!("oss error: {}", &error_message.content());
        }
    }
    Ok(())
}
