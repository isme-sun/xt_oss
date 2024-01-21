use xt_oss::{oss, utils};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = utils::options_from_env();
    let client = oss::Client::new(options);
    // * GetBucketPolicy用于获取指定存储空间（Bucket）的权限策略（Policy）
    let result = client.GetBucketPolicy().await;
    match result {
        Ok(data) => {
            println!("{:#?}", data.headers);
        }
        Err(message) => {
            println!("{}", message)
        }
    }
}
