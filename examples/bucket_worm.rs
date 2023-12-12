use xt_oss::{oss, utils};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = utils::options_from_env();
    let client = oss::Client::new(options);
    // * 创建WORM规则 *
    // let result = client.InitiateBucketWorm().days(1).send().await;
    // match result {
    //     Ok(data) => {
    //         println!("{:#?}", data.headers);
    //     }
    //     Err(message) => {
    //         println!("{}", message)
    //     }
    // }

    // let result = client
    //     .CompleteBucketWorm("47184D37AFE04D59BB64480EBA6A9402")
    //     .await;
    // match result {
    //     Ok(data) => {
    //         println!("{:#?}", data.headers);
    //     }
    //     Err(message) => {
    //         println!("{}", message)
    //     }
    // }

    // * 终止WORM规则 */
    // let result = client.AbortBucketWorm().await;
    // match result {
    // 	Ok(data) => {
    // 		println!("{:#?}", data.headers);
    // 	},
    // 	Err(message) => {
    // 		println!("{}", message)
    // 	}
    // }

    // * 获取WORM规则 */
    let result = client.GetBucketWorm().await;
    match result {
        Ok(data) => {
            println!("{}", serde_json::to_string(&data.data).unwrap());
        }
        Err(message) => {
            println!("{}", message)
        }
    }
}
