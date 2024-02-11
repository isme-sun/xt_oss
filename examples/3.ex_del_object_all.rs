use std::{process, sync::Arc};

use dotenv;
use futures::future::join_all;
use xt_oss::{oss, utils};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  dotenv::dotenv().ok();
  let options = utils::options_from_env();
  let client = Arc::new(oss::Client::new(options));
  loop {
    match client
      .ListObjectsV2()
      .with_max_keys(5)
      .execute()
      .await
      .unwrap_or_else(|error| {
        println!("reqwest error: {}", error);
        process::exit(-1);
      }) {
      Ok(data) => {
        let data = data.content();
        if let Some(objects) = data.contents {
          let tasks = objects.into_iter().map(|object| {
            let client = Arc::clone(&client);
            async move {
              let result = client.DeleteObject(object.key.as_str()).execute().await;
              match result {
                Ok(Ok(_)) => println!("delete object key: {}", object.key),
                Ok(Err(message)) => println!("{}", message.content()),
                Err(error) => println!("reqwest error: {}", error),
              }
            }
          });
          join_all(tasks).await;
        } else {
          println!("bucket file clean");
          return Ok(());
        }
      }
      Err(message) => {
        println!("oss error: {}", message.content())
      }
    }
  }
}
