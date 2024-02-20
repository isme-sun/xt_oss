use std::process;

use xt_oss::{
    oss::{self},
    util,
};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);
    match client
        .DeleteMultipleObjects()
        .execute()
        .await
        .unwrap_or_else(|reqwest_error| {
            eprintln!("{}", reqwest_error);
            process::exit(-1);
        }) {
        Ok(oss_data) => {
            println!("{:#?}", oss_data.headers());
            println!("{:#?}", oss_data.content());
        }
        Err(error_message) => {
            println!("{:#?}", error_message.content())
        }
    }
}
