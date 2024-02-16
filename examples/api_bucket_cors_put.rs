use dotenv;
use std::process;
use xt_oss::{
    oss::{
        self,
        entities::cors::builder::{
            AllowedHeaderItem, AllowedMethodItem, AllowedOriginItem, CORSConfigurationBuilder,
            CORSRuleBuilder,
        },
        http,
    },
    utils,
};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = utils::options_from_env();
    let client = oss::Client::new(options);

    let cors_rule1 = CORSRuleBuilder::new()
        .with_allowed_origin(AllowedOriginItem::Urls(vec![
            "https://localhost:3000",
            "http://localhost:3001",
        ]))
        .with_allowed_header(AllowedHeaderItem::Any)
        .with_allowed_method(AllowedMethodItem::Any)
        .with_max_age_seconds(30)
        .builder();

    let cors_rule2 = CORSRuleBuilder::new()
        .with_allowed_origin(AllowedOriginItem::Urls(vec!["https://dev.example.local"]))
        .with_allowed_header(AllowedHeaderItem::Headers(vec![
            http::header::CONTENT_ENCODING,
            http::header::CONTENT_LENGTH,
            http::header::CONTENT_RANGE,
        ]))
        .with_allowed_method(AllowedMethodItem::Methods(vec![
            http::Method::GET,
            http::Method::POST,
        ]))
        .with_expose_header(vec!["x-oss-test", "x-oss-test1"])
        .with_max_age_seconds(60)
        .builder();

    let cors_config = CORSConfigurationBuilder::new()
        .add_rule(cors_rule1)
        .add_rule(cors_rule2)
        .with_response_vary(false)
        .builder();

    let result = client
        .PutBucketCors()
        .with_config(cors_config)
        .execute()
        .await
        .unwrap_or_else(|reqwest_error| {
            println!("reqwest error: {}", reqwest_error);
            process::exit(-1);
        });

    match result {
        Ok(oss_data) => {
            println!("{:#?}", oss_data.headers())
        }
        Err(error_message) => {
            println!("{}", error_message.content())
        }
    }
}
