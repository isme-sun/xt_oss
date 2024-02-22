use chrono::{Duration, Utc};
use dotenv;
use std::process;
use xt_oss::oss::http::ContentDisposition::ATTACHMENT;
use xt_oss::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);

    let object = "ppt/File-1000kb.ppt";
    let content_disposition = ATTACHMENT(Some(object.into())).to_string();
    let expire = util::utc_to_gmt(Utc::now() + Duration::days(1));
    let cache_control = http::CacheControl::NoCache.to_string();
    // Retrieve 500 bytes starting from the 100th byte
    let range = ByteRange::from((100, 500));

    match client
        .GetObject("ppt/File-1000kb.ppt")
        .with_content_disposition(&content_disposition)
        .with_expires(&expire)
        .with_cache_control(&cache_control)
        .with_range(range)
        .with_content_encoding("gzip")
        .with_accept_encoding("zh-CN")
        .with_timeout(120)
        .execute()
        .await
        .unwrap_or_else(|error| {
            eprintln!("reqwest error: {}", error);
            process::exit(-1);
        }) {
        Ok(oss_data) => {
            println!("{:#?}", oss_data.headers());
            println!("content len: {}", oss_data.content().len())
        }
        Err(error_message) => {
            println!("{:#?}", error_message.content())
        }
    }
    Ok(())
}
