use xt_oss::{oss, utils};

// * ex: * 调用InitiateBucketWorm接口新建一条合规保留策略
#[allow(unused)]
async fn ex_initiate_bucket_worm() {
    dotenv::dotenv().ok();
    let options = utils::options_from_env();
    let client = oss::Client::new(options);

    let result = client.InitiateBucketWorm().days(1).send().await;
    match result {
        Ok(result) => {
            println!("status: {}", result.status);
            println!("headers: {:#?}", result.headers);
            println!("data: {:#?}", result.data);
            // let content = serde_json::to_string_pretty(&result.data).unwrap();
        }
        Err(message) => {
            println!("{}", message)
        }
    }
}

// * ex: * CompleteBucketWorm用于锁定合规保留策略
#[allow(unused)]
async fn ex_complete_bucket_worm() {
    dotenv::dotenv().ok();
    let options = utils::options_from_env();
    let client = oss::Client::new(options);

    let result = client
        .CompleteBucketWorm("BC15D1A1AD0D48AD97EC096D87D705BD")
        .await;
    match result {
        Ok(data) => {
            println!("{:#?}", data.headers);
        }
        Err(message) => {
            println!("{}", message)
        }
    }
}

// * ex: * 终止WORM规则
#[allow(unused)]
async fn ex_abort_bucket_worm() {
    dotenv::dotenv().ok();
    let options = utils::options_from_env();
    let client = oss::Client::new(options);

    let result = client.AbortBucketWorm().await;
    match result {
        Ok(result) => {
            println!("result.status: {}", result.status);
            println!("result.headers: {:#?}", result.headers);
            println!("result.data: {:#?}", result.data);
        }
        Err(message) => {
            println!("{}", message)
        }
    }
}

// * 获取WORM规则 */
#[allow(unused)]
async fn ex_get_bucket_worm() {
    dotenv::dotenv().ok();
    let options = utils::options_from_env();
    let client = oss::Client::new(options);

    let result = client.GetBucketWorm().await;
    match result {
        Ok(result) => {
            println!("status: {}", result.status);
            println!("headers: {:#?}", result.headers);
            println!("data: {:#?}", result.data);
            // let content = serde_json::to_string_pretty(&result.data).unwrap();
        }
        Err(message) => {
            println!("{}", message)
        }
    }
}

// * ex: * 获取WORM规则
#[allow(unused)]
async fn ex_extend_bucket_worm() {
    dotenv::dotenv().ok();
    let options = utils::options_from_env();
    let client = oss::Client::new(options);

    let worm_id = "BC15D1A1AD0D48AD97EC096D87D705BD";
    let result = client
        .ExtendBucketWorm()
        .worm_id(worm_id)
        .days(2)
        .send()
        .await;
    match result {
        Ok(data) => {
            println!("{:#?}", data.data);
        }
        Err(message) => {
            println!("{:#?}", message)
        }
    }
}

#[tokio::main]
async fn main() {
    // * ex: * 调用InitiateBucketWorm接口新建一条合规保留策略
    ex_initiate_bucket_worm().await;
    // * ex: * CompleteBucketWorm用于锁定合规保留策略
    // ex_complete_bucket_worm().await;
    // * 终止WORM规则 */
    // ex_abort_bucket_worm().await;
    // * 获取WORM规则 */
    // ex_get_bucket_worm().await;
    // * ExtendBucketWorm用于延长已锁定的合规保留策略对应Bucket中Object的保留天数。*
    // ex_extend_bucket_worm().await;
}
