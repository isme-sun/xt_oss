use std::time::{SystemTime, UNIX_EPOCH};

use reqwest::header::{HeaderMap, HeaderValue, DATE};

#[tokio::main]
async fn main() {
  let date = format!(
    "{}",
    SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap()
      .as_secs()
  );

  let mut headers = HeaderMap::new();
  headers.insert(DATE, HeaderValue::from_str(&date).unwrap());
  println!("{:#?}", headers);
}
