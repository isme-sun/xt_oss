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
    {
        Ok(Ok(oss_data)) => {
            let version_config = oss_data.content();
            if let Some(status) = version_config.status {
                println!("version status: {}", status);
            } else {
                println!("Version feature not enabled");
            }
        }
        Ok(Err(oss_error_message)) => {
            println!("{:#?}", oss_error_message.content());
        }
        Err(reqwest_error) => {
            println!("reqwest error {}", reqwest_error);
            process::exit(-1);
        }
    };
    Ok(())
}
