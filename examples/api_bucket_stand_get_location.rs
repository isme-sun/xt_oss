use dotenv;
use std::process;
use xt_oss::{oss::{self, entities::bucket::LocationConstraint}, utils};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = utils::options_from_env();
    let client = oss::Client::new(options);
    let result = client
        .GetBucketLocation()
        .with_bucket("xtoss-t1")
        .execute()
        .await
        .unwrap_or_else(|error| {
            println!("reqwest error: {}", error);
            process::exit(-1);
        });

    match result {
        Ok(data) => {
            let LocationConstraint(location) = data.content();
            println!("location: {}", location);
        }
        Err(error) => {
            println!("{}", error.url());
            println!("{:#?}", error.content())
        }
    }
}

