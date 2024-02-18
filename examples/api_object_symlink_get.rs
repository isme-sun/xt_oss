use std::process;

use xt_oss::{oss, utils};

#[tokio::main]
async fn main() {
  dotenv::dotenv().ok();
  let options = utils::options_from_env();
  let client = oss::Client::new(options);

  match client
    .GetSymlink("tmp/test.xls")
    .execute()
    .await
    .unwrap_or_else(|error| {
      println!("{}", error);
      process::exit(-1);
    }) {
    Ok(oss_data) => {
      println!("{:#?}", &oss_data.headers())
    }
    Err(error_message) => {
      println!("{:#?}", &error_message.content())
    }
  }
}
