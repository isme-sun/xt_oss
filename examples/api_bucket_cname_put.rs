use dotenv;
#[allow(unused)]
use std::process;
#[allow(unused)]
use xt_oss::prelude::*;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    // let options = utils::options_from_env();
    // let client = oss::Client::new(options);
    // let result = client
    //   .GetBucketInfo()
    //   .with_bucket("xuetube")
    //   .execute()
    //   .await
    //   .unwrap_or_else(|error| {
    //     println!("reqwest error: {}", error);
    //     process::exit(-1);
    //   });

    // match result {
    //   Ok(data) => {
    //     println!("{:#?}", data.content)
    //   }
    //   Err(error) => {
    //     println!("{}", error.content)
    //   }
    // }
}
