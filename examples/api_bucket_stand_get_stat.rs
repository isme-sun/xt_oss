use dotenv;
use serde_json;
use std::process;
use xt_oss::{oss, utils};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = utils::options_from_env();
    let client = oss::Client::new(options);
    let result = client
        .GetBucketStat()
        .with_region("oss-cn-beijing")
        .with_bucket("xtoss-t1")
        .execute()
        .await
        .unwrap_or_else(|error| {
            println!("reqwest error: {}", error);
            process::exit(-1);
        });

    match result {
        Ok(data) => {
            println!("{}", serde_json::to_string_pretty(&data.content()).unwrap());
        }
        Err(error) => {
            println!("{}", error.url());
            println!("{:#?}", error.content())
        }
    }
}
