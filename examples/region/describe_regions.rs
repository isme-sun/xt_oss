use std::process;

use xt_oss::oss::api;
use xt_oss::{oss, utils};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = utils::options_from_env();
    let client = oss::Client::new(options);
    let resp = client
        .DescribeRegions()
        // .with_region("oss-sn-shanghai")
        .with_timeout(30)
        .execute()
        .await;

    let result = resp.unwrap_or_else(|error| {
        println!("request error:{}", error);
        process::exit(-1);
    });

    let data = if let api::ResponseKind::SUCCESS(result) = result {
        result
    } else {
        println!("oss error!");
        println!("{:#?}", result);
        process::exit(-1);
    };
    let data = data.content();
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
}
