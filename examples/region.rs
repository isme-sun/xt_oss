use dotenv;
use serde_json;

use xt_oss::oss;
use xt_oss::utils;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = utils::options_from_env();
    let client = oss::Client::new(options);
    let result = client.DescribeRegions().send().await.unwrap();
    let content = serde_json::to_string_pretty(&result.data).unwrap();
    println!("{}", content);
}
