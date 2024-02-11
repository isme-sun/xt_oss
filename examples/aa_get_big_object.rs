#[allow(unused)]
use std::process;

use xt_oss::{oss, utils};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = utils::options_from_env();
    let client = oss::Client::new(options);
    println!("{:#?}", client);
    Ok(())
}
