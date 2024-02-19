use std::process;

use xt_oss::{
    oss::{self},
    utils,
};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = utils::options_from_env();
    let client = oss::Client::new(options);
    match client
        .DeleteObject("tmp/test.txt")
        .with_version_id("CAEQ2AEYgYCA1v6ot.sYIiBmZjU2NTQwOGEwZDc0MTMyYTU5ZjhlMmUyNGYwMjc3NA--")
        .execute()
        .await
        .unwrap_or_else(|reqwest_error| {
            eprintln!("{}", reqwest_error);
            process::exit(-1);
        }) {
        Ok(oss_data) => {
            println!("{:#?}", oss_data.headers())
        }
        Err(error_message) => {
            println!("{:#?}", error_message.content())
        }
    }
}
