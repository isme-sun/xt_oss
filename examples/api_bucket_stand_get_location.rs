use dotenv;
use std::process;
use xt_oss::{
  oss::{self, entities::bucket::LocationConstraint},
  utils,
};

#[tokio::main]
async fn main() {
  dotenv::dotenv().ok();
  let options = utils::options_from_env();
  let client = oss::Client::new(options);
  let result = client
    .GetBucketLocation()
    .with_bucket("xtoss-t1")
    .execute()
    .await
    .unwrap_or_else(|reqwest_error| {
      println!("reqwest error: {}", reqwest_error);
      process::exit(-1);
    });

  match result {
    Ok(oss_data) => {
      let LocationConstraint(location) = oss_data.content();
      println!("location: {}", location);
    }
    Err(oss_error) => {
      println!("{}", oss_error.url());
      println!("{:#?}", oss_error.content())
    }
  }
}
