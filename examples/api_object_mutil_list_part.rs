use dotenv;
use std::process;
use xt_oss::prelude::*;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);
    match client
        .ListParts("tmp/temp.jpg")
        .with_upload_id("E71E2C09F952430F93700A3167F74685")
        .execute()
        .await
        .unwrap_or_else(|reqwest_error| {
            eprintln!("reqwest error: {}", reqwest_error);
            process::exit(-1);
        }) {
        Ok(oss_data) => {
            println!("{:#?}", oss_data.content())
        }
        Err(error_message) => {
            println!("{:#?}", error_message.content())
        }
    }
}
