use std::process;

use dotenv;
use xt_oss::utils;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    match utils::options_from_env()
        .client()
        .ListObjectVersions()
        // .with_max_keys(5)
        // .with_prefix("mp3")
        .with_delimiter("/")
        // .with_key_marker(value)
        // .with_encoding_type(value)
        // .with_version_id_marker(value)
        .execute()
        .await
        .unwrap_or_else(|error| {
            println!("reqwest error {}", error);
            process::exit(-1);
        }) {
        Ok(data) => {
            let version_objects = data.content();
            // println!("{:#?}", version_objects);
            println!(
                "{}",
                serde_json::to_string_pretty(&version_objects).unwrap()
            );
        }
        Err(oss_error_message) => {
            println!("{:#?}", oss_error_message.content());
        }
    };
    Ok(())
}
