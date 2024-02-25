//! `cargo run --example api_bucket_acl_put -q`
//! PutBucketAcl接口用于设置或修改存储空间`Bucket`的访问权限`ACL`。
//!
//! - [official docs](https://help.aliyun.com/zh/oss/developer-reference/putbucketacl)
//! - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_acl_put.rs)
use std::process;

use xt_oss::oss::entities::OssAcl;
use xt_oss::prelude::*;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);

    match client
        .PutBucketAcl(OssAcl::PublicRead)
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
}
