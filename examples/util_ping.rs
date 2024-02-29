use dotenv;
use xt_oss::{oss, util};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
		let options = util::options_from_env();
		let client = oss::Client::new(options);

		// 创建一个 10M 大小的向量，用 0 填充
		let data: Vec<u8> = vec![0; 30 * 1024 * 1024];
		let object = "tmp/data.tmp";
		let result = client.PutObject(object)
			.with_content(oss::Bytes::from(data))
			.with_timeout(30)
			.execute().await?;

		match result {
			Ok(data) => {
				println!("{:#?}",data);
			},
			Err(message) => {
				println!("{:#?}", message);
			}
		}

    Ok(())
}
