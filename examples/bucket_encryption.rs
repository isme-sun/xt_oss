#[allow(unused)]
use xt_oss::{
	oss::{self, entities::encryption::SSEAlgorithm},
	utils,
};

#[tokio::main]
async fn main() {
	dotenv::dotenv().ok();
	let options = utils::options_from_env();
	let client = oss::Client::new(options);

	// * PutBucketEncryption接口用于配置存储空间（Bucket）的加密规则。
	// let result = client
	//     .PutBucketEncryption()
	//     .algorithm(SSEAlgorithm::SM4)
	//     .send()
	//     .await;

	// match result {
	//     Ok(data) => {
	//         println!("{:#?}", data.headers);
	//     }
	//     Err(message) => {
	//         println!("{}", message)
	//     }
	// }

	// * GetBucketEncryption接口用于获取存储空间（Bucket）的加密规则 *
	// let result = client.GetBucketEncryption().await;
	// match result {
	// 	Ok(data) => {
	// 		println!("{:#?}", data);
	// 	},
	// 	Err(message) => {
	// 		println!("{:#?}", message)
	// 	}
	// }

	// * DeleteBucketEncryption接口用于删除Bucket加密规则 *
	let result = client.DeleteBucketEncryption().await;
	match result {
		Ok(data) => {
			println!("{:#?}", data);
		}
		Err(message) => {
			println!("{:#?}", message)
		}
	}
}
