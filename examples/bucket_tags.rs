use xt_oss::{oss, utils};

#[tokio::main]
async fn main() {
	dotenv::dotenv().ok();
	let options = utils::options_from_env();
	let client = oss::Client::new(options);
	// * 设置 *
	println!("{:#?}", client.options());
	let result = client
		.PutBucketTags()
		.add_tag("version", "1.0.0")
		.add_tag("name", "xuetube-dev")
		.add_tag("env", "dev")
		.add_tag("desc", "职业教育")
		.send()
		.await;
	println!("{:#?}", result);

	// * 获取*
	// let result = client.GetBucketTags().await;
	// match result {
	//     Ok(resp) => {
	//         println!("{}", serde_json::to_string(&resp.data).unwrap());
	//     }
	//     Err(message) => println!("{:#?}", message),
	// }

	// * 删除 *
	// let result = client
	//     .DeleteBucketTags()
	//     .delete_key("desc")
	//     .delete_key("env")
	//     .send()
	//     .await;
	// match result {
	//     Ok(resp) => {
	//         println!("{:#?}", resp)
	//     }
	//     Err(message) => println!("{:#?}", message),
	// }
}
