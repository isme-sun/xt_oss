use xt_oss::{oss, utils};

async fn create_bucket() {
    let options = utils::options_from_env();
    let client = oss::Client::new(options);
    let result = client.PutBucket().name("xuetube-t11").send().await.unwrap();
    println!("{:#?}", result);
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    create_bucket().await;
}
