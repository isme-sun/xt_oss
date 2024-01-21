
use xt_oss::{
	oss::{self, entities::ObjectACL},
	utils,
};

#[allow(unused)]
async fn get_object_acl(client: oss::Client<'_>) {
	let result = client.GetObjectACL("xtoss/example/settings.json").await;
	match result {
		Ok(result) => {
			println!("{:#?}", result.data);
		}
		Err(message) => {
			println!("{:#?}", message);
		}
	}
}

#[allow(unused)]
async fn put_object_acl(client: oss::Client<'_>) {
	let result = client
		.PutObjectACL("xtoss/example/settings.json")
		.acl(ObjectACL::PublicRead)
		.send()
		.await;
	match result {
		Ok(result) => {
			println!("{:#?}", result.data);
		}
		Err(message) => {
			println!("{:#?}", message);
		}
	}
}

#[tokio::main]
async fn main() {
	dotenv::dotenv().ok();
	let options = utils::options_from_env();
	let client = oss::Client::new(options);
	get_object_acl(client).await;
	// put_object_acl(client).await;
}
