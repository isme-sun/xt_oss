//! cargo run --package xt-oss --example api_bucket_cname_put -q
//!
//! 调用PutCname接口为某个存储空间(Bucket)绑定自定义域名
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/putcname)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_cname_put.rs)
use dotenv;
use std::process;
use xt_oss::{oss::entities::cname::builders::BucketCnameConfigurationBuilder, prelude::*};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);
    // 配置项构建
    // let config = BucketCnameConfigurationBuilder::new()
    //     .with_domain("example.com")
    //     .with_cert_id("493****-cn-hangzhou")
    //     .with_certificate("-----BEGIN CERTIFICATE----- certificate -----END CERTIFICATE-----")
    //     .with_private_key("-----BEGIN CERTIFICATE----- private_key -----END CERTIFICATE-----<")
    //     .with_previous_cert_id("493****-cn-hangzhou")
    //     .with_force(true)
    //     .build();
    // 测试域名改成自己的域名
    let cname_domain = "xtoss-web.example.com";
    let config = BucketCnameConfigurationBuilder::new()
        .with_domain(cname_domain)
        .build();

    let result = client
        .PutCname()
        .with_config(config)
        .execute()
        .await
        .unwrap_or_else(|error| {
            println!("reqwest error: {}", error);
            process::exit(-1);
        });
    match result {
        Ok(data) => {
            println!("{:#?}", data.content())
        }
        Err(error) => {
            println!("{:#?}", error.content())
        }
    }
    Ok(())
}
