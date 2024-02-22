use chrono::{Days, Utc};
use dotenv;
use std::process;
use xt_oss::{oss::http::ContentDisposition, prelude::*};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);

    let content_disposition = ContentDisposition::ATTACHMENT(Some("文件.ppt".into())).to_string();

    let expire = Utc::now()
        .checked_add_days(Days::new(1))
        .unwrap()
        .format(oss::GMT_DATE_FMT)
        .to_string();

    let cache_control = http::CacheControl::NoCache.to_string();

    match client
        .GetObject("ppt/File-1000kb.ppt")
        .with_content_disposition(&content_disposition)
        .with_expires(&expire)
        .with_cache_control(&cache_control)
        .with_range(ByteRange::from((100, 500)))
        .with_content_encoding("gzip")
        .with_accept_encoding("zh-CN")
        .with_timeout(120)
        .execute()
        .await
        .unwrap_or_else(|error| {
            println!("reqwest error: {}", error);
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
