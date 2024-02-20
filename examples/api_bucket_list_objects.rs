use dotenv;
use std::process;
use xt_oss::{oss, util};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);
    let result = client
        .ListObjects()
        .with_max_keys(2)
        .with_prefix("course/video")
        .with_marker("course/video/00518bfd279de57ea6a8b26a5af2c0fc/content.mp4")
        .execute()
        .await
        .unwrap_or_else(|error| {
            println!("reqwest error: {}", error);
            process::exit(-1);
        });

    match result {
        Ok(data) => {
            println!("{:#?}", data.content())
        }
        Err(error) => {
            println!("{:#?}", error.content())
        }
    }
}
