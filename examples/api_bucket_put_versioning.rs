use dotenv;
use std::process;
use xt_oss::{oss::entities::version::VersioningStatus, prelude::*};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);
    let result = client
        .PutBucketVersioning(VersioningStatus::Enabled)
        .execute()
        .await
        .unwrap_or_else(|error| {
            println!("reqwest error: {}", error);
            process::exit(-1);
        });

    match result {
        Ok(data) => {
            println!("{:#?}", data.url());
            println!("{:#?}", data.status());
            println!("{:#?}", data.headers());
            println!("{:#?}", data.content())
        }
        Err(error) => {
            println!("{:#?}", error.content())
        }
    }
    Ok(())
}
