use xt_oss::{oss, utils};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = utils::options_from_env();
    let client = oss::Client::new(options);
    let result = client.GetBucketReferer().await;
    match result {
        Ok(result) => {
					println!("{:#?}", result);
					println!("-----------------------");
					println!("referer_list");
					for url in result.data.referer_list {
						println!("{}", url);
					}
					println!("-----------------------");
					println!("referer_blacklist");
					for url in result.data.referer_blacklist {
						println!("{}", url);
					}
					println!("-----------------------");
        },
        Err(message) => {
            println!("{}", message);
        }
    }
}
