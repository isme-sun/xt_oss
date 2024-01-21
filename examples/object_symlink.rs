use dotenv;
use xt_oss::{
	oss::{
		self,
		entities::{ObjectACL, StorageClass},
	},
	utils,
};

#[allow(unused)]
async fn put_symlink() {
	let options = utils::options_from_env();
	let client = oss::Client::new(options);
	let result = client
		.PutSymlink()
		.with_object("tmp.jpg")
		.with_symlink_target("res/test.jpeg")
		.with_forbid_overwrite(true)
		.with_object_acl(ObjectACL::PublicRead)
		.with_storage_class(StorageClass::Standard)
		.send()
		.await;
	match result {
		Ok(result) => {
			println!("{:#?}", result);
		}
		Err(message) => {
			println!("{:#?}", message);
		}
	}
}

#[allow(unused)]
async fn get_symlink() {
	let options = utils::options_from_env();
	let client = oss::Client::new(options);
	let result = client.GetSymlink().with_object("tmp.jpg").send().await;

	match result {
		Ok(result) => {
			println!("{:#?}", result);
		}
		Err(message) => {
			println!("{:#?}", message);
		}
	}
}

#[tokio::main]
async fn main() {
	// todo 测试url编码
	dotenv::dotenv().ok();
	// put_symlink().await;
	get_symlink().await;
}
