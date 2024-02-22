use dotenv;
use std::process;
use xt_oss::prelude::*;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);
    let result = client
        .ListObjectsV2()
        .with_max_keys(10)
        // .with_prefix("course/video")
        .execute()
        .await
        .unwrap_or_else(|error| {
            println!("reqwest error: {}", error);
            process::exit(-1);
        });

    match result {
        Ok(data) => {
            println!("{}", serde_json::to_string_pretty(&data.content()).unwrap())
        }
        Err(error) => {
            println!("{:#?}", error.content())
        }
    }
}
