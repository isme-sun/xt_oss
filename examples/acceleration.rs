use xt_oss::{oss, utils};

/// 获取cname信息
#[allow(unused)]
async fn get_bucket_transfer_acceleration(client: &oss::Client<'_>) {
    let result = client.GetBucketTransferAcceleration().await;
    match result {
        Ok(resp) => {
            println!("{:#?}", resp);
        }
        Err(message) => {
            println!("{:#?}", message);
        }
    }
}

#[allow(unused)]
async fn put_bucket_transfer_acceleration(client: &oss::Client<'_>) {
    let result = client.PutBucketTransferAcceleration(false).await;
    match result {
        Ok(resp) => {
            println!("{:#?}", resp);
        }
        Err(message) => {
            println!("{:#?}", message);
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = utils::options_from_env();
    let client = oss::Client::new(options);
    get_bucket_transfer_acceleration(&client).await;
    // put_bucket_transfer_acceleration(&client).await;
}
