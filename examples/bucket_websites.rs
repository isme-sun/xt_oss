use xt_oss::{oss, utils};

#[allow(unused)]
async fn put_bucket_website() {
    dotenv::dotenv().ok();
    let options = utils::options_from_env();
    let client = oss::Client::new(options);

    let result = client.PutBucketWebsite().with_default().send().await;

    match result {
        Ok(result) => {
            println!("{:#?}", result);
        }
        Err(message) => {
            println!("{:?}", message);
        }
    }
}

#[allow(unused)]
async fn get_bucket_website() {
    dotenv::dotenv().ok();
    let options = utils::options_from_env();
    let client = oss::Client::new(options);

    let result = client.GetBucketWebsite().await;

    match result {
        Ok(result) => {
            println!("[status]: {}", result.status);
            println!("[headers]:{:#?}", result.headers);
            println!("[data]:{:#?}", result.data);
            println!(
                "json: {}",
                serde_json::to_string_pretty(&result.data).unwrap()
            )
        }
        Err(message) => {
            println!("{:?}", message);
        }
    }
}

#[allow(unused)]
async fn delete_bucket_website() {
    dotenv::dotenv().ok();
    let options = utils::options_from_env();
    let client = oss::Client::new(options);

    let result = client.DeleteBucketWebsite().await;

    match result {
        Ok(result) => {
            println!("{:#?}", result);
        }
        Err(message) => {
            println!("{:?}", message);
        }
    }
}

#[tokio::main]
async fn main() {
    // put_bucket_website().await;
    // get_bucket_website().await;
    delete_bucket_website().await;
}
