use dotenv;
use std::process;
use xt_oss::{oss, util};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);
    let result = client.GetBucketCors().execute().await.unwrap_or_else(|reqwest_error| {
        println!("reqwest error: {}", reqwest_error);
        process::exit(-1);
    });

    match result {
        Ok(oss_data) => {
            println!("{:#?}", oss_data.content())
        }
        Err(error_message) => {
            println!("{}", error_message.content())
        }
    }
}
