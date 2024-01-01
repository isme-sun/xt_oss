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

    // 7A4AFCD96C394FDBAF9CFF9107507750
    // * CompleteBucketWorm用于锁定合规保留策略 *
    // let result = client
    //     .CompleteBucketWorm("BC15D1A1AD0D48AD97EC096D87D705BD")
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
            println!("{:#?}", data.data);
        }
        Err(message) => {
            println!("{}", message)
        }
    }

    // * ExtendBucketWorm用于延长已锁定的合规保留策略对应Bucket中Object的保留天数。*
    // let worm_id = "BC15D1A1AD0D48AD97EC096D87D705BD";
    // let result = client.ExtendBucketWorm()
    //                    .worm_id(worm_id)
    //                    .days(2)
    //                    .send()
    //                    .await;
    // match result {
    //     Ok(data) => {
    //         println!("{:#?}", data.data);
    //     }
    //     Err(message) => {
    //         println!("{:#?}", message)
    //     }
    // }
}
