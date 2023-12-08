use dotenv;

use xt_oss::oss;
use xt_oss::utils;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = utils::options_from_env();
    let client = oss::Client::new(options);
    let rs = client.ListBuckets().send().await.unwrap();
    println!("{}", serde_json::to_string(&rs.data).unwrap());
}
