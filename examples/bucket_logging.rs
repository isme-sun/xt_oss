use xt_oss::{oss, utils};

#[allow(unused)]
async fn del_bucket_logging() {
    let options = utils::options_from_env();
    let client = oss::Client::new(options);
    let resp = client.DeleteBucketLogging().await;

    match resp {
        Ok(logging_status) => println!("{:#?}", logging_status),
        Err(message) => println!("{:#?}", message),
    }
}

#[allow(unused)]
async fn get_bucket_logging() {
    let options = utils::options_from_env();
    let client = oss::Client::new(options);
    let resp = client.GetBucketLogging().await;

    match resp {
        Ok(logging_status) => {
            println!(
                "{}",
                serde_json::to_string_pretty(&logging_status.data).unwrap()
            );
        }
        Err(message) => println!("{:#?}", message),
    }
}

#[allow(unused)]
async fn put_bucket_logging() {
    let options = utils::options_from_env();
    let client = oss::Client::new(options);
    let resp = client
        .PutBucketLogging()
        .with_enabled(true)
        .with_target_prefix("sunjy")
        .send()
        .await;
    println!("{:#?}", resp);
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    // put_bucket_logging().await;
    // get_bucket_logging().await;
    del_bucket_logging().await;
}
