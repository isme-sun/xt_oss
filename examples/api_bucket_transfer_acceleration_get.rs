use std::process;

use dotenv;
use xt_oss::{oss::entities::acceleration::TransferAccelerationConfiguration, util};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    match util::options_from_env()
        .client()
        .GetBucketTransferAcceleration()
        // .PutBucketVersioning(VersioningStatus::Suspended)
        .execute()
        .await
        .unwrap_or_else(|error| {
            println!("reqwest error {}", error);
            process::exit(-1);
        }) {
        Ok(oss_data) => {
            let TransferAccelerationConfiguration { enabled } = oss_data.content();
            println!("enabled: {}", enabled);
        }
        Err(oss_error_message) => {
            println!("{:#?}", oss_error_message.content());
        }
    };
    Ok(())
}
