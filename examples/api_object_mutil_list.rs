use dotenv;
use std::process;
use xt_oss::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);
    match client
        .ListMultipartUploads()
        .execute()
        .await
        .unwrap_or_else(|reqwest_error| {
            eprintln!("{}", reqwest_error);
            process::exit(-1);
        }) {
        Ok(oss_data) => {
            println!("{:#?}", oss_data.content())
        }
        Err(error_message) => {
            println!("{:#?}", error_message.content())
        }
    }
    Ok(())
}
