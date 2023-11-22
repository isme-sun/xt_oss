#[allow(unused_imports)]
use chrono::Utc;
#[allow(unused_imports)]
use dotenv::dotenv;
#[allow(unused_imports)]
use reqwest::{self, header, header::HeaderValue, Method, Request, Url};
#[allow(unused_imports)]
use std::time::Duration;
#[allow(unused_imports)]
use xt_oss::common::OssOptions;
#[allow(unused_imports)]
use xt_oss::params::ListBucketsQuery;
#[allow(unused_imports)]
use xt_oss::utils::get_gmt_date;
#[allow(unused_imports)]
use xt_oss::OssClient;
// use xt_oss::utils::hmac_sha1;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let client = OssClient::builder(OssOptions::from_env());
    let result = client
        .ListBuckets(ListBucketsQuery {
            prefix: Some("xu".to_string()),
            marker: Some("xue".to_string()),
            max_keys: Some(2),
        })
        .await
        .unwrap();
    println!("{:#?}", result);
    Ok(())
}
