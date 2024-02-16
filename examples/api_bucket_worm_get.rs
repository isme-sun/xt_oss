use std::process;
use dotenv;
use xt_oss::{oss, utils};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
		dotenv::dotenv().ok();
    let options = utils::options_from_env();
    let client = oss::Client::new(options);
    let result = client
        .GetBucketWorm()
        .execute()
        .await
        .unwrap_or_else(|reqwest_error| {
            println!("reqwest {}", reqwest_error);
            process::exit(-1);
        });
    match result {
        Ok(oss_data) => {
            println!("{:#?}", oss_data.content());
        }
        Err(error_message) => {
            println!("oss error: {}", error_message.content());
        }
    }

    Ok(())
}