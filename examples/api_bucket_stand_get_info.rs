use dotenv;
use serde_json;
use std::process;
use xt_oss::prelude::*;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);
    let result = client
        .GetBucketInfo()
        .with_bucket("xtoss-t1")
        .execute()
        .await
        .unwrap_or_else(|error| {
            println!("reqwest error: {}", error);
            process::exit(-1);
        });

    match result {
        Ok(oss_data) => {
            println!("{}", serde_json::to_string_pretty(&oss_data.content()).unwrap());
        }
        Err(oss_error_message) => {
            println!("{:#?}", oss_error_message.content())
        }
    }
}
