use xt_oss::{oss, utils};

#[tokio::main]
async fn main() {
	dotenv::dotenv().ok();
	let options = utils::options_from_env();
	let client = oss::Client::new(options);

	// let result = client.PutStyle()
	//     .name("style4")
	//     .content("image/resize,p_150")
	//     .send().await;
	// match result {
	//     Ok(result) => {
	//         println!("{:#?}", result)
	//     },
	//     Err(message) => {
	//         println!("{:#?}", message)
	//     }
	// }

	let result = client.ListStyle().await;
	match result {
		Ok(result) => {
			// println!("{:#?}", result.data);
			println!("{}", serde_json::to_string(&result.data).unwrap())
		}
		Err(message) => {
			println!("{:#?}", message)
		}
	}

	// let result = client.GetStyle("style1").await;
	// match result {
	//     Ok(result) => {
	//         println!("{}", serde_json::to_string(&result.data).unwrap())
	//     },
	//     Err(message) => {
	//         println!("{:#?}", message)
	//     }
	// }

	// let result = client.DeleteStyle("style2").await;
	// match result {
	//     Ok(result) => {
	//         println!("{:#?}", result)
	//     },
	//     Err(message) => {
	//         println!("{:#?}", message)
	//     }
	// }
}
