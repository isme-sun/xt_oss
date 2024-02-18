use std::process;

use xt_oss::{oss, utils};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = utils::options_from_env();
    let client = oss::Client::new(options);
    let resp = client
        .HeadObject("tmp/database_book.pdf")
        // .with_version_id(version_id)
        // .with_match(value)
        // .with_modified_since(value)
        // .with_none_match(value)
        // .with_unmodified_since(value)
        .execute()
        .await
        .unwrap_or_else(|error| {
            println!("reqwest error: {}", error);
            process::exit(-1);
        });
    match resp {
        Ok(data) => {
            println!("{:#?}", data.headers())
        }
        Err(message) => {
            println!("{:#?}", message.content())
        }
    }
}
