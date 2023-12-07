use dotenv;
use serde_json;

use xt_oss::oss;
use xt_oss::utils;

/// 获取cname信息
async fn get_cname() {
    dotenv::dotenv().ok();
    let options = utils::options_from_env();
    let client = oss::Client::new(options);
    let result = client.ListCname().await.unwrap();
    let content = serde_json::to_string_pretty(&result.data).unwrap();
    println!("{}", content);
}

#[tokio::main]
async fn main() {
    get_cname().await;
}
