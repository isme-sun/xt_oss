use std::process;

use xt_oss::{oss, utils};

#[tokio::main]
async fn main() {
  dotenv::dotenv().ok();
  let options = utils::options_from_env();
  let client = oss::Client::new(options);

  match client
    .GetObjectACL("excel/Spreadsheet-1000-rows.xls")
    .with_version_id("CAEQmgEYgYDA9I_smO0YIiBhOGJmMTczNzY0ZmM0NTE1YTA5MDJlOWE1YmI1ZTZlNQ--")
    .execute()
    .await
    .unwrap_or_else(|error| {
      println!("{}", error);
      process::exit(-1);
    }) {
    Ok(oss_data) => {
      println!("{:#?}", oss_data.content())
    }
    Err(error_message) => {
      println!("{:#?}", error_message.content())
    }
  }
}
