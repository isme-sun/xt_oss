use std::process;

use xt_oss::{
  oss::{self, entities::OssAcl},
  utils,
};

#[tokio::main]
async fn main() {
  dotenv::dotenv().ok();
  let options = utils::options_from_env();
  let client = oss::Client::new(options);

  match client
    .PutBucketAcl(OssAcl::PublicReadWrite)
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
