//! ` cargo run --package xt-oss --example api_bucket_policy_put -q`
//!
//! PutBucketPolicy接口用于为指定的存储空间`Bucket`设置授权策略`Policy`。
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/putbucketpolicy)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_policy_put.rs)
use dotenv;
use std::process;
use xt_oss::prelude::*;

const POLICY_TEXT: &'static str = r#"{
    "Version": "1",
    "Statement": [
      {
        "Principal": [ "*" ],
        "Effect": "Allow",
        "Resource": [ "acs:oss:*:1508492296054765:xtoss-ex1/tmp/*" ],
        "Action": [ "oss:GetObject", "oss:GetObjectAcl", "oss:ListObjects" ]
      },
      {
        "Principal": [ "*" ],
        "Effect": "Allow",
        "Resource": [ "acs:oss:*:1508492296054765:xtoss-ex1" ],
        "Condition": {
          "StringLike": {
            "oss:Prefix": [ "tmp/*" ]
          }
        },
        "Action": [ "oss:ListObjects", "oss:GetObject" ]
      }
    ]
  }"#;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);
    let result = client
        .PutBucketPolicy()
        .with_policy(POLICY_TEXT)
        .execute()
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
            println!("{}", error_message.content())
        }
    }
    Ok(())
}
