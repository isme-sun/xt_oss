use std::process;

use xt_oss::{oss, utils};

#[tokio::main]
async fn main() {
  dotenv::dotenv().ok();
  let options = utils::options_from_env();
  let client = oss::Client::new(options);

  match client
    .PutObjectTagging("excel/Spreadsheet-1000-rows.xls")
    .with_tag("key1", "value1")
    .with_tag("key2", "value2")
    .with_tag("key3", "value3")
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
