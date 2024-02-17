use dotenv;
use reqwest::StatusCode;
use std::process;
use xt_oss::{
    oss::{
        self,
        entities::website::builder::{
            ErrorDocumentBuilder, IndexDocumentBuilder, WebsiteConfigurationBuilder,
        },
    },
    utils,
};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = utils::options_from_env();
    let client = oss::Client::new(options);

    let index_document = IndexDocumentBuilder::new()
        .with_suffix("index.html")
        .with_support_sub_dir(true)
        .with_type(1)
        .build();

    let error_document = ErrorDocumentBuilder::new()
        .with_http_status(StatusCode::NOT_FOUND)
        .with_key("error.html")
        .build();

    let config = WebsiteConfigurationBuilder::new()
        .with_index_document(index_document)
        .with_error_document(error_document)
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
