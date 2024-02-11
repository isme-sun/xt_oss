use std::{env, fs::File, io::Read, process};

use chrono::Utc;
use xt_oss::{
  oss::{self},
  utils,
};

#[tokio::main]
async fn main() {
  dotenv::dotenv().ok();
  let options = utils::options_from_env();
  let client = oss::Client::new(options);

  println!("{}", env::current_dir().unwrap().to_str().unwrap());
  let mut content = vec![];
  let mut file = File::open("examples/assets/database_book.pdf").unwrap();
  let _ = file.read_to_end(&mut content);
  let content = oss::Bytes::from(content);

  let resp = client
    .PutObject("tmp/database_book.pdf")
    .with_content(content)
    .with_expires(Utc::now())
    .with_content_type("application/pdf")
    .with_oss_tagging("name", "图书")
    .with_oss_tagging("cate", "测试")
    .with_oss_meta("origin-name", "database-book")
    .execute()
    .await
    .unwrap_or_else(|error| {
      println!("{}", error);
      process::exit(-1);
    });

  match resp {
    Ok(data) => {
      println!("{:#?}", data.headers())
    }
    Err(message) => {
      println!("{:#?}", message.content())
    }
  }
}
