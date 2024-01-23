#[allow(unused)]
use xt_oss::{
    oss::{
        self,
        entities::cors::builder::{CORSConfigurationBuilder, CORSRuleBuilder},
    },
    utils,
};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = utils::options_from_env();
    let client = oss::Client::new(options);

    // * 调用PutBucketCors接口为指定的存储空间（Bucket）设置跨域资源共享CORS（Cross-Origin Resource Sharing）规则
    let rule1 = CORSRuleBuilder::new()
        .allowed_origin("https://localhost:3000")
        .allowed_method(oss::Method::GET)
        .allowed_method(oss::Method::PUT)
        .allowed_method(oss::Method::POST)
        .allowed_method(oss::Method::DELETE)
        .allowed_header(oss::header::AUTHORIZATION)
        .max_age_seconds(100)
        .builder();

    let rule2 = CORSRuleBuilder::new()
        .allowed_origin("https://localhost:8076")
        .allowed_method(oss::Method::GET)
        .allowed_method(oss::Method::PUT)
        .allowed_method(oss::Method::POST)
        .allowed_method(oss::Method::DELETE)
        .allowed_header(oss::header::AUTHORIZATION)
        .max_age_seconds(100)
        .builder();

    let config = CORSConfigurationBuilder::new()
        .add_rule(rule1)
        .add_rule(rule2)
        .response_vary(false)
        .builder();

    let result = client.PutBucketCors().config(config).send().await;
    match result {
        Ok(result) => {
            println!("{:#?}", result)
        }
        Err(message) => {
            println!("{:#?}", message)
        }
    }

    // * GetBucketCors接口用于获取指定存储空间（Bucket）当前的跨域资源共享CORS（Cross-Origin Resource Sharing）规则
    // let result = client.GetBucketCors().await;
    // match result {
    //     Ok(result) => {
    //         println!("{}", serde_json::to_string(&result.data).unwrap());
    //     }
    //     Err(message) => {
    //         println!("{:#?}", message)
    //     }
    // }

    // * DeleteBucketCors用于关闭指定存储空间（Bucket）对应的跨域资源共享CORS（Cross-Origin Resource Sharing）功能并清空所有规则
    // let result = client.DeleteBucketCors().await;
    // match result {
    //     Ok(result) => {
    //         println!("{:#?}", result)
    //     },
    //     Err(message) => {
    //         println!("{:#?}", message)
    //     }
    // }
}
