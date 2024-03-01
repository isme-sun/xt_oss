//! `cargo run --package xt-oss --example api_bucket_payment_put`
//! PutBucketRequestPayment接口用于设置请求者付费模式。
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/putbucketrequestpayment)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_payment_put.rs)
use dotenv;
use std::process;
use xt_oss::{oss::entities::payment::Payer, prelude::*};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);
    let result = client
        .PutBucketRequestPayment(Payer::BucketOwner)
        .exectue()
        .await
        .unwrap_or_else(|reqwest_error| {
            println!("reqwest error: {}", reqwest_error);
            process::exit(-1);
        });

    match result {
        Ok(oss_data) => {
            println!("{:#?}", oss_data.headers());
        }
        Err(error_message) => {
            println!("{:#?}", error_message.content())
        }
    }
    Ok(())
}
