use futures::future::join_all;
use std::sync::Arc;
use xt_oss::{oss, utils};

#[tokio::main]
async fn main() {
  dotenv::dotenv().ok();
  let options = utils::options_from_env();
  let client = Arc::new(oss::Client::new(options));
  let region = "oss-cn-beijing".to_string();
  let buckets = (10..20)
    .map(|i| format!("xtoss-ex{}", i))
    .collect::<Vec<String>>();

  let futures = buckets.into_iter().map(|bucket| {
    let client = Arc::clone(&client);
    let region = region.clone();
    async move {
      match client
        .PutBucket()
        .with_bucket(&bucket)
        .with_region(&region)
        .execute()
        .await
      {
        Ok(Ok(_)) => println!("create {}@{} is success", &bucket, &region),
        Ok(Err(error)) => println!("{:#?}", error.content()),
        Err(error) => println!("reqwest error: {}", error),
      }
    }
  });

  join_all(futures).await;
}
