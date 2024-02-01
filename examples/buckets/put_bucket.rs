use dotenv;
use std::{option, process};

use xt_oss::{
  oss::{
    self,
    entities::{DataRedundancyType, OssAcl, StorageClass},
    Options,
  },
  utils,
};

#[tokio::main]
async fn main() {
  dotenv::dotenv().ok();
  let key_id = std::env::var("OSS_ACCESS_KEY_ID")
    .unwrap_or_default()
    .leak();
  let key_secret = std::env::var("OSS_ACCESS_KEY_SECRET")
    .unwrap_or_default()
    .leak();
  let sts_token = std::env::var("OSS_STS_TOKEN").unwrap_or_default().leak();
  let endpoint = std::env::var("OSS_ENDPOINT").unwrap_or_default().leak();
  let region = std::env::var("OSS_REGION").unwrap_or_default().leak();
  let bucket = std::env::var("OSS_BUCKET").unwrap_or_default().leak();
  let internal = std::env::var("OSS_INTERNAL")
    .unwrap_or_default()
    .parse::<bool>()
    .unwrap_or_default();
  let is_request_pay = std::env::var("OSS_IS_REQUEST_PAY")
    .unwrap_or_default()
    .parse()
    .unwrap_or_default();
  let secure = std::env::var("OSS_SECURE")
    .unwrap_or_default()
    .parse()
    .unwrap_or_default();
  let cname = std::env::var("OSS_CNAME")
    .unwrap_or_default()
    .parse()
    .unwrap_or_default();
  let timeout = std::env::var("OSS_TIMEOUT")
    .unwrap_or_default()
    .parse()
    .unwrap_or_default();

  let options = Options::new()
    .with_access_key_id(key_id)
    .with_access_key_secret(key_secret)
    .with_bucket(bucket)
    .with_sts_token(sts_token)
    .with_endpoint(endpoint)
    .with_region(region)
    .with_internal(internal)
    .with_is_request_pay(is_request_pay)
    .with_secret(secure)
    .with_cname(cname)
    .with_timeout(timeout);

  /*

  OSS_ACCESS_KEY_ID
  OSS_ACCESS_KEY_SECRET
  OSS_STS_TOKEN
  OSS_ENDPOINT
  OSS_REGION
  OSS_BUCKET
  OSS_INTERNAL
  OSS_IS_REQUEST_PAY
  OSS_SECURE
  OSS_CNAME
  OSS_TIMEOUT=
     */

  // let options = utils::options_from_env();
  let client = oss::Client::new(options);
  let result = client
    .PutBucket()
    .with_region("oss-cn-beijing")
    .with_bucket("xtoss-t4")
    .with_acl(OssAcl::PublicRead)
    // .with_group_id("your_group_name")
    .with_storage_class(StorageClass::Archive)
    .with_data_redundancy_type(DataRedundancyType::LRS)
    .execute()
    .await
    .unwrap_or_else(|error| {
      println!("reqwest error: {}", error);
      process::exit(-1);
    });

  match result {
    Ok(data) => {
      println!("{:#?}", data)
    }
    Err(error) => {
      println!("{:#?}", error)
    }
  }
}
