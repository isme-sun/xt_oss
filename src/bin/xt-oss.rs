use dotenv::dotenv;
#[allow(unused)]
use urlencoding;
use xt_oss::common::OssOptions;
#[allow(unused_imports)]
use xt_oss::params::{DescribeRegionsQuery, ListBucketsQuery, ListObject2Query};
use xt_oss::OssClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let client = OssClient::builder(OssOptions::from_env());
    let bucket_info = client.GetBucketInfo().await.unwrap();
    let json_str = serde_json::to_string(&bucket_info.data).unwrap();
    println!("{}", json_str);
    // println!("{}", serde_json::to_string(&bucket_info).unwrap());



    // let stat = client.GetBucketStat()
    //     .await
    //     .unwrap();
    // println!("{}", serde_json::to_string(&stat.data).unwrap());
    // ------------
    // let mut query = ListObject2Query::default();
    // query.prefix = Some("course/video".to_string());
    // query.max_keys = Some(20);
    // let result = client
    //     .ListObjectsV2(query)
    //     .await
    //     .unwrap();
    // for object in &result.data.contents {
    //     println!("{} {}", object.size, urlencoding::decode(&object.key).unwrap());
    // }
    // println!("{}", serde_json::to_string(&result.data).unwrap());
    // ------------
    // let retval = client
    //     .DescribeRegions(DescribeRegionsQuery::default())
    //     .await
    //     .unwrap();
    // println!("{:#?}", retval.data);
    Ok(())
}
