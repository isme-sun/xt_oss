use dotenv;
use std::process;
use xt_oss::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);

    let object = "tmp/temp.jpg";
    let upload_id = "E71E2C09F952430F93700A3167F74685";

    match client
        .CompleteMultipartUpload(object)
        .with_upload_id(upload_id)
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
