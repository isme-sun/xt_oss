use std::process;

use dotenv;
use xt_oss::{oss, util};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);
    match client
        .ExtendBucketWorm("933141599A8941FD9592F24F9862A5DE")
        .with_days(2)
        .execute()
        .await
        .unwrap_or_else(|error| {
            println!("reqwest error {}", error);
            process::exit(-1);
        }) {
        Ok(oss_data) => {
            println!("{:#?}", oss_data.headers());
        }
        Err(oss_error_message) => {
            println!("{:#?}", oss_error_message.content());
        }
    };
    Ok(())
}
