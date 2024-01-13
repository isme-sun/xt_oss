use dotenv;

use xt_oss::oss;
use xt_oss::utils;

/// 获取cname信息
async fn get_cname() {
    dotenv::dotenv().ok();
    let options = utils::options_from_env();
    let client = oss::Client::new(options);

    let result = client.ListCname().await;
    match result {
        Ok(result) => {
            println!("{:#?}", result.data)
        }
        Err(message) => {
            println!("{:#?}", message)
        }
    }
}

#[tokio::main]
async fn main() {
    get_cname().await;
}