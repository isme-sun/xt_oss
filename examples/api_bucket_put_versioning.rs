use dotenv;
use std::process;
use xt_oss::{
    oss::{self, entities::version::VersioningStatus},
    util,
};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);
    let result = client
        .PutBucketVersioning(VersioningStatus::Enabled)
        .execute()
        .await
        .unwrap_or_else(|error| {
            println!("reqwest error: {}", error);
            process::exit(-1);
        });

    match result {
        Ok(data) => {
            println!("{:#?}", data.url());
            println!("{:#?}", data.status());
            println!("{:#?}", data.headers());
            println!("{:#?}", data.content())
        }
        Err(error) => {
            println!("{:#?}", error.content())
        }
    }
}
