//! `cargo run --example api_bucket_lifecycle_put -q`
//!
//! 您可以基于最后一次修改时间以及最后一次访问时间的策略创建生命周期规则，定期将存储空间
//! `Bucket`内的多个文件`Object`转储为指定存储类型,或者将过期的Object和碎片删除,
//! 从而节省存储费用。本文为您介绍如何调用PutBucketLifecycle接口为存储空间`Bucket``
//! 设置生命周期规则。
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/putbucketlifecycle)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_lifecycle_put.rs)
use dotenv;
use std::process;
use xt_oss::{
    oss::entities::{
        lifecycle::builder::{ExpirationBuilder, LifecycleConfigurationBuilder, RuleBuilder},
        Status,
    },
    prelude::*,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);

    let expire_180_day = ExpirationBuilder::new().with_days(180).build();
    let expire_30_day = ExpirationBuilder::new().with_days(30).build();

    let rule1 = RuleBuilder::new()
        .with_id("Rule1")
        .with_prefix("mp3")
        // Status default Diabled
        // .with_status(Status::Enabled)
        .with_expiration(expire_180_day)
        .build();

    let rule2 = RuleBuilder::new()
        .with_id("Rule2")
        .with_prefix("excel")
        .with_status(Status::Enabled)
        .with_expiration(expire_30_day)
        .build();

    let config = LifecycleConfigurationBuilder::new()
        .with_rule(rule1)
        .with_rule(rule2)
        .build();

    match client
        .PutBucketLifecycle()
        .with_config(config)
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
