use dotenv;
use std::process;

#[allow(unused)]
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
  let options: Options = utils::options_from_env();
  println!("{:#?}", options);
  let client = oss::Client::new(options);
  let result = client
    .PutBucket()
    .with_region("oss-cn-beijing")
    .with_bucket("xtoss-ex5")
    .with_acl(OssAcl::PublicRead)
    .with_storage_class(StorageClass::Standard)
    // .with_data_redundancy_type(DataRedundancyType::LRS)
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
