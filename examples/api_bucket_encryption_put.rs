use dotenv;
use std::process;
use xt_oss::{
    oss::{self, entities::encryption::SSEAlgorithm},
    util,
};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);
    let result = client
        .PutBucketEncryption()
        .with_algorithm(SSEAlgorithm::KMS)
        // .with_data_encryption("SM4")
        // .with_master_key_id("--your value --")
        .execute()
        .await
        .unwrap_or_else(|reqwest_error| {
            println!("reqwest error: {}", reqwest_error);
            process::exit(-1);
        });

    match result {
        Ok(oss_data) => {
            println!("{:#?}", oss_data.headers())
        }
        Err(error_message) => {
            println!("{}", error_message.content())
        }
    }
}
