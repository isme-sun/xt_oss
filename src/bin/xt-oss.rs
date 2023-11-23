use dotenv::dotenv;
use xt_oss::common::OssOptions;
#[allow(unused_imports)]
use xt_oss::params::{DescribeRegionsQuery, ListBucketsQuery, ListObject2Query};
use xt_oss::OssClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let client = OssClient::builder(OssOptions::from_env());
    // let stat = client.GetBucketStat()
    //     .await
    //     .unwrap();
    // println!("{}", serde_json::to_string(&stat.data).unwrap());
    // ------------
    let mut query = ListObject2Query::default();
    query.max_keys = Some(5);
    let _ = client
        .ListObjectsV2(query)
        .await;
        // .unwrap();
    // println!("{:?}", oss_data);
    // ------------
    // let retval = client
    //     .DescribeRegions(DescribeRegionsQuery::default())
    //     .await
    //     .unwrap();
    // println!("{:#?}", retval.data);
    Ok(())
}
