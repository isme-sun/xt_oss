use dotenv;
use std::process;
use xt_oss::{oss::entities::object::TaggingDirective, prelude::*};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);

    let tags = [("k1", "v1"), ("k2", "v2"), ("k3", "v3")].to_vec();

    match client
        .CopyObject("tmp/tmp/copy_test_index.html")
        .with_copy_source("/xtoss-ex10/index.html")
        .with_oss_tagging(tags)
        .with_tagging_directive(TaggingDirective::REPLACE)
        .execute()
        .await
        .unwrap_or_else(|error| {
            println!("reqwest error: {}", error);
            process::exit(-1);
        }) {
        Ok(oss_data) => {
            println!("{:#?}", oss_data.content());
        }
        Err(error_message) => {
            println!("{:#?}", error_message.content())
        }
    }
    Ok(())
}
