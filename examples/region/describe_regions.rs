use xt_oss::oss::api::Error::{OssError, ReqwestError};
use xt_oss::{oss, utils};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = utils::options_from_env();
    let client = oss::Client::new(options);
    let resp = client
        .DescribeRegions()
        // .with_region("oss-sn-shanghai")
        .with_timeout(30)
        .execute()
        .await;

    match resp {
        Ok(data) => {
            println!("{}", serde_json::to_string_pretty(data.content()).unwrap());
        }
        Err(error) => match error {
            ReqwestError(error) => println!("{}", error),
            OssError(error) => println!("{:#?}", error),
        },
    }
    // println!("{:#?}", resp);
}
