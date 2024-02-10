use std::process;

use dotenv;
use xt_oss::{oss, utils};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  dotenv::dotenv().ok();
  let options = utils::options_from_env();
  let client = oss::Client::new(options);

  let mut token: Option<String> = None;
  let mut count = 0u32;
  let mut size = 0u64;
  let mut page = 1;
  loop {
    match client
      .ListObjectsV2()
      .with_max_keys(5)
      // .with_prefix("txt")
      // .with_encoding_type("url")
      .with_continuation_token(token.as_deref())
      .execute()
      .await
      .unwrap_or_else(|error| {
        println!("reqwest error: {}", error);
        process::exit(-1);
      }) {
      Ok(data) => {
        let objects = data.content();
        if objects.key_count == Some(0) {
          println!("not object");
        } else {
          token = objects.next_continuation_token.clone();
          println!("-- PAGE {} --", page);
          for object in objects.contents.unwrap() {
            size = size + object.size as u64;
            println!(" - {}", object.key);
          }
        }
        count = count + objects.key_count.unwrap();
        page = page + 1;
      }
      Err(message) => {
        println!("oss error {}:", message.url());
        println!("oss error {:#?}:", message.content());
        break;
      }
    }
    if token.is_none() {
      break;
    }
  }

  println!("{}", "-".repeat(60));
  println!("file count: {}", count);
  println!("size: {:.2}MB", (size as f64) / 1024f64 / 1024f64);

  Ok(())
}
