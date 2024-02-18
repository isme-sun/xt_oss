use std::process;

use chrono::{Days, Utc};
use xt_oss::{
    oss::{self, entities::ContentDisposition},
    utils::{self, ByteRange},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = utils::options_from_env();
    let client = oss::Client::new(options);

    let content_disposition = ContentDisposition::ATTACHMENT(Some("文件.ppt".to_string())).to_string();

    let expire = Utc::now()
        .checked_add_days(Days::new(1))
        .unwrap()
        .format(oss::GMT_DATE_FMT)
        .to_string();

    let resp = client
        .GetObject("ppt/File-1000kb.ppt")
        .with_content_disposition(&content_disposition)
        .with_expires(&expire)
        .with_range(ByteRange(Some(100), Some(500)))
        .execute()
        .await
        .unwrap_or_else(|error| {
            println!("reqwest error: {}", error);
            process::exit(-1);
        });
    match resp {
        Ok(data) => {
            println!("{:#?}", data.headers());
            println!("content len: {}", data.content().len())
        }
        Err(message) => {
            println!("{:#?}", message.content())
        }
    }
    Ok(())
}
