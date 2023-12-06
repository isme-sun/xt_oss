use xt_oss::oss;
use xt_oss::oss::arguments::DescribeRegionsQuery;
use xt_oss::utils;

#[tokio::main]
async fn main() {
		dotenv::dotenv().ok();
    let options = utils::options_from_env();
    let client = oss::Client::new(options);
    let region = DescribeRegionsQuery::default();
    let result = client.DescribeRegions(region).await.unwrap();
    let content = serde_json::to_string(&result.data).unwrap();
    println!("{}", content);
}
