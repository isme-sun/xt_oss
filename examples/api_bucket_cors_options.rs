//! # Options
//!
//! 浏览器在发送跨域请求之前会发送一个preflight请求（Options）给OSS，并带上特定的来源域、HTTP方法和header等信息，
//! 以决定是否发送真正的请求。Options请求是由浏览器自动根据是否跨域来决定是否发送。
use dotenv;
use std::process;
// use xt_oss::{
//     oss::{self, entities::cors::builder::AllowedHeaderItem, http},
//     util,
// };
use xt_oss::{prelude::*, util::AllowedHeaderItem};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);
    let result = client
        .Options("test.txt")
        .with_origin("https://dev.example.local")
        .with_request_method(http::Method::POST)
        .with_request_headers(AllowedHeaderItem::Headers(vec![
            http::header::CONTENT_ENCODING,
            http::header::CONTENT_LENGTH,
            http::header::CONTENT_RANGE,
        ]))
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
