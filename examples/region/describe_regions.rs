use std::process;

// use xt_oss::oss::api;
use xt_oss::{oss, utils};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = utils::options_from_env();
    let client = oss::Client::new(options);
    let result = client
        .DescribeRegions()
        .with_region("oss-sn-shanghai")
        .with_timeout(30)
        .execute()
        .await
        .unwrap_or_else(|error| {
            println!("request error:{}", error);
            process::exit(-1);
        });

    let data = result.unwrap();
    println!("{}", serde_json::to_string_pretty(&data.content()).unwrap());
}
