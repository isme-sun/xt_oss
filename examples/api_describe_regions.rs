use dotenv;
use std::process;
use xt_oss::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);
    match client
        .DescribeRegions()
        // .with_region("oss-us-east-1")
        .execute()
        .await
        .unwrap_or_else(|reqwest_error| {
            println!("reqweset error: {}", reqwest_error);
            process::exit(-1);
        }) {
        Ok(oss_data) => {
            oss_data
                .content()
                .region_info
                .into_iter()
                .for_each(|entry| {
                    println!("{:>20} | {}", entry.region, entry.internet_endpoint);
                });
        }
        Err(oss_error_message) => println!("oss error: {}", oss_error_message.content()),
    }
    Ok(())
}
