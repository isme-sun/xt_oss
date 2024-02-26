use dotenv;
use reqwest::StatusCode;
use std::process;
use xt_oss::{
    oss::entities::website::builder::{
        ErrorDocumentBuilder, IndexDocumentBuilder, WebsiteConfigurationBuilder,
    },
    prelude::*,
};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);

    // 构建website配置
    let index_document = IndexDocumentBuilder::new()
        .with_suffix("index.html")
        .with_support_sub_dir(true)
        .with_type(0)
        .build();

    let error_document = ErrorDocumentBuilder::new()
        .with_http_status(StatusCode::NOT_FOUND)
        .with_key("error.html")
        .build();

    // let rules = RoutingRulesBuilder::new()
    //     .with_rule(
    //         RoutingRuleBuilder::new()
    //             .with_rule_number(1)
    //             .with_condition(ConditionBuilder::new().build())
    //             .with_redirect(RedirectBuilder::new().build())
    //             .build(),
    //     )
    //     .with_rule(
    //         RoutingRuleBuilder::new()
    //             .with_rule_number(2)
    //             .with_condition(ConditionBuilder::new().build())
    //             .with_redirect(RedirectBuilder::new().build())
    //             .build(),
    //     )
    //     .build();

    let config = WebsiteConfigurationBuilder::new()
        .with_index_document(index_document)
        .with_error_document(error_document)
        // .with_routing_rules(rules)
        .build();

    let result = client
        .PutBucketWebsite()
        .with_config(config)
        .execute()
        .await
        .unwrap_or_else(|reqwest_error| {
            println!("reqwest error: {}", reqwest_error);
            process::exit(-1);
        });

    match result {
        Ok(oss_data) => {
            println!("{:#?}", oss_data.headers());
        }
        Err(error_message) => {
            println!("{}", error_message.content())
        }
    }
}
