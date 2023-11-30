#[allow(unused_imports)]
use std::io::Write;
#[allow(unused_imports)]
use std::process;

use dotenv::dotenv;
#[allow(unused_imports)]
use xt_oss::arguments::{DescribeRegionsQuery, ListBucketsQuery, ListObject2Query};
use xt_oss::{OssClient, OssOptions};
// use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let option = OssOptions::from_env();
    let client = OssClient::builder(option);
    // ***********************************************************************
    // let rs = client
    //     .GetObject("upload/2022/05/2d3b8dc1-6955-40de-a23b-21a1389d218f.jpg".to_string())
    //     .await
    //     .unwrap_or_else(|err| {
    //         println!("{}", err);
    //         process::exit(-1);
    //     });

    // let data = rs.data;
    // let mut file = std::fs::File::create("data.jpg").expect("create failed");
    // let rs = file.write_all(&data).expect("write failed");
    // println!("{:#?}", rs);
    // ***********************************************************************
    // let rs = client
    //     .HeadObject("upload/2022/05/2d3b8dc1-6955-40de-a23b-21a1389d218f.jpg".to_string())
    //     .await
    //     .unwrap_or_else(|err| {
    //         println!("{}", err);
    //         process::exit(-1);
    //     });

    // println!("{:#?}", rs);

    // let headers = rs.headers;
    // for key in headers.keys() {
    //     let key_name = key.as_str();
    //     let value = headers.get(key).unwrap().to_str().unwrap();
    //     println!("{} : {}", key_name, value);
    // }
    // ***********************************************************************
    // let rs = client
    //     .GetObjectMeta("upload/2022/05/2d3b8dc1-6955-40de-a23b-21a1389d218f.jpg".to_string())
    //     .await
    //     .unwrap_or_else(|err| {
    //         println!("{}", err);
    //         process::exit(-1);
    //     });

    // let headers = rs.headers;
    // for key in headers.keys() {
    //     let key_name = key.as_str();
    //     let value = headers.get(key).unwrap().to_str().unwrap();
    //     println!("{} : {}", key_name, value);
    // }

    // ***********************************************************************
    // let bucket_info = client.GetBucketInfo().await.unwrap();
    // let json_str = serde_json::to_string(&bucket_info.data).unwrap();
    // println!("{}", json_str);
    // ***********************************************************************
    // let stat = client.GetBucketStat()
    //     .await
    //     .unwrap();
    // println!("{}", serde_json::to_string(&stat.data).unwrap());
    // ***********************************************************************
    // let mut query = ListObject2Query::default();
    // query.prefix = Some("course/video".to_string());
    // query.max_keys = Some(20);
    // let result = client
    //     .ListObjectsV2(query)
    //     .await
    //     .unwrap();

    // //  for object in &result.data.contents {
    // //     println!("{} {}", object.size, urlencoding::decode(&object.key).unwrap());
    // // }
    // println!("{}", serde_json::to_string(&result.data).unwrap());
    // ***********************************************************************
    // let query = DescribeRegionsQuery::default();
    // // query.regions = Some("oss-cn-hangzhou".to_string());
    // let result = client.DescribeRegions(query).await;
    // match result {
    //     Ok(result) => {
    //         let json_str = serde_json::to_string(&result.data).unwrap();
    //         print!("{}", json_str);
    //     },
    //     Err(err) => {
    //         let json_str = serde_json::to_string(&err).unwrap();
    //         println!("{}", json_str);
    //     }
    // }
    // ***********************************************************************
    // let retval = client.ListCname().await.unwrap();
    // let json_str = serde_json::to_string(&retval.data).unwrap();
    // print!("{}", json_str);
    // ***********************************************************************
    let query = ListBucketsQuery::default();
    let retval = client.ListBuckets(query).await.unwrap();
    // println!("{:#?}",retval);
    let json_str = serde_json::to_string(&retval.data).unwrap();
    print!("{}", json_str);
    Ok(())
}
