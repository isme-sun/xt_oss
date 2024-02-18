use std::process;

use xt_oss::{oss, utils};

#[tokio::main]
async fn main() {
  dotenv::dotenv().ok();
  let options = utils::options_from_env();
  let client = oss::Client::new(options);

  match client
    .DeleteObjectTagging("excel/Spreadsheet-1000-rows.xls")
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
