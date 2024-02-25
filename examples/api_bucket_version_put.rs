use dotenv;
use std::process;
use xt_oss::{oss::entities::version::VersioningStatus, prelude::*};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    match util::options_from_env()
        .client()
        // .PutBucketVersioning(VersioningStatus::Enabled)
        .PutBucketVersioning(VersioningStatus::Suspended)
        .execute()
        .await
        .unwrap_or_else(|error| {
            println!("reqwest error {}", error);
            process::exit(-1);
        }) {
        Ok(_) => {
            println!("success")
        }
        Err(oss_error_message) => {
            println!("{:#?}", oss_error_message.content());
        }
    };
    Ok(())
}
