use std::process;

use xt_oss::{oss, utils};

#[tokio::main]
async fn main() {
  dotenv::dotenv().ok();
  let options = utils::options_from_env();
  let client = oss::Client::new(options);
  let resp = client
    .GetObjectMeta("xtoss/example/123.png")
		// .with_version_id("abc123")
    .execute()
    .await
    .unwrap_or_else(|error| {
      println!("reqwest error: {}", error);
      process::exit(-1);
    });
  match resp {
    Ok(data) => {
      println!("{:#?}", data.headers)
    }
    Err(message) => {
      println!("{:#?}", message.content)
    }
  }
}
