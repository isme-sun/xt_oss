use dotenv;
use std::process;
use xt_oss::{oss, util};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);
    let result = client
        .InitiateBucketWorm()
        .with_days(1)
        .execute()
        .await
        .unwrap_or_else(|reqwest_error| {
            println!("reqwest {}", reqwest_error);
            process::exit(-1);
        });
    match result {
        Ok(oss_data) => {
            println!("{:#?}", oss_data.headers());
        }
        Err(error_message) => {
            println!("oss error: {}", error_message.content());
        }
    }

    Ok(())
}
