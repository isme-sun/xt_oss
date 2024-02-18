use dotenv;
use std::process;
use xt_oss::{oss, utils};

#[tokio::main]
async fn main() {
  dotenv::dotenv().ok();
  let options = utils::options_from_env();
  let client = oss::Client::new(options);
  let result = client
    .DeleteStyle("cover")
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
}
