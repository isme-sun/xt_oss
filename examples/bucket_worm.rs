use xt_oss::{oss, utils};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = utils::options_from_env();
    let client = oss::Client::new(options);
		// let result = client.InitiateBucketWorm().days(5).send().await;
		// match result {
		// 	Ok(data) => {
		// 		println!("{:#?}", data.headers);
		// 	},
		// 	Err(message) => {
		// 		println!("{}", message)
		// 	}
		// }
		let result = client.GetBucketWorm().await;
		match result {
			Ok(data) => {
				println!("{:#?}", data.data);
			},
			Err(message) => {
				println!("{}", message)
			}
		}

}
