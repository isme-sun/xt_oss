use std::process;

use dotenv;
use xt_oss::utils;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    match utils::options_from_env()
        .client()
        .GetBucketVersioning()
        .execute()
        .await
        .unwrap_or_else(|error| {
            println!("reqwest error {}", error);
            process::exit(-1);
        }) {
        Ok(oss_data) => {
            let version_config = oss_data.content();
            if let Some(status) = version_config.status {
                println!("version status: {}", status);
            } else {
                println!("Version feature not enabled");
            }
        }
        Err(oss_error_message) => {
            println!("{:#?}", oss_error_message.content());
        }
    };
    Ok(())
}
