use xt_oss::oss;
use xt_oss::oss::arguments::ListBucketsQuery;
use xt_oss::utils;

#[tokio::main]
async fn main() {
		dotenv::dotenv().ok();
    let options = utils::options_from_env();
    let client = oss::Client::new(options);
		let query = ListBucketsQuery::default();
    let result = client.ListBuckets(query).await.unwrap();
    let content = serde_json::to_string(&result.data).unwrap();
    println!("{}", content);
}