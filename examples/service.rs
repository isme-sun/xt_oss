

use xt_oss::oss;
use xt_oss::utils;

#[tokio::main]
async fn main() {
	dotenv::dotenv().ok();
	let options = utils::options_from_env();
	let client = oss::Client::new(options);
	let rs = client.ListBuckets().send().await.unwrap();

	// for bucket in &rs.data.buckets.bucket {
	//     println!("{:#?}",bucket.creation_date);
	// }

	println!("{}", serde_json::to_string_pretty(&rs.data).unwrap());
}
