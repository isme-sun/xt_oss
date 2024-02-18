use std::process;

use dotenv;
use xt_oss::utils;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  dotenv::dotenv().ok();
  match utils::options_from_env()
    .client()
    // .PutBucketTransferAcceleration(true)
    .PutBucketTransferAcceleration(false)
    .execute()
    .await
    .unwrap_or_else(|error| {
      println!("reqwest error {}", error);
      process::exit(-1);
    }) {
    Ok(_) => {
      println!("success")
    }
    Err(oss_error_message) => {
      println!("{:#?}", oss_error_message.content());
    }
  };
  Ok(())
}
