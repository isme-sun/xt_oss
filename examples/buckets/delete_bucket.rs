use dotenv;
use std::process;
use xt_oss::{oss, utils};

#[tokio::main]
async fn main() {
  dotenv::dotenv().ok();
  let options = utils::options_from_env();
  let client = oss::Client::new(options);
  let result = client
    .DeleteBucket()
    .with_region("oss-cn-beijing")
    .with_bucket("xtoss-t4")
    .execute()
    .await
    .unwrap_or_else(|error| {
      println!("reqwest error: {}", error);
      process::exit(-1);
    });

  match result {
    Ok(data) => {
      println!("{:#?}", data.url());
      println!("{:#?}", data.status());
      println!("{:#?}", data.headers());
      println!("{:#?}", data.content)
    }
    Err(error) => {
      println!("{:#?}", error.content)
    }
  }
}
