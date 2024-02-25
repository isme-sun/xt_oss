use dotenv;
use std::process;
use xt_oss::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);

    match client
        .GetObjectTagging("excel/Spreadsheet-1000-rows.xls")
        .execute()
        .await
        .unwrap_or_else(|error| {
            println!("{}", error);
            process::exit(-1);
        }) {
        Ok(data) => {
            println!("{}", serde_json::to_string_pretty(&data.content())?);
        }
        Err(message) => {
            println!("{:#?}", message.content())
        }
    }

    Ok(())
}
