use dotenv;
use std::process;
use xt_oss::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);

    let objects = client
        .ListObjectsV2()
        .with_max_keys(5)
        .execute()
        .await?
        .unwrap()
        .content()
        .contents;

    if let Some(objects) = objects {
        let objects: Vec<(&str, &str)> = objects.iter().map(|e| (e.key.as_str(), "")).collect();
        match client
            .DeleteMultipleObjects()
            .with_deletes(objects)
            .execute_quiet()
            // .execute()
            .await
            .unwrap_or_else(|reqwest_error| {
                eprintln!("{}", reqwest_error);
                process::exit(-1);
            }) {
            Ok(oss_data) => {
                println!("{:#?}", oss_data.headers());
                println!("{:#?}", oss_data.content());
            }
            Err(error_message) => {
                println!("{:#?}", error_message.content());
            }
        }
    } else {
        println!("bucket is empty");
    }

    Ok(())
}
