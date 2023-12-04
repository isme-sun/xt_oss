use std::{thread::sleep, time::Duration};

use reqwest::Method;
// use std::{thread::sleep, time::Duration};
// use xt_oss::utils::options_from_env;
use xt_oss::{oss, utils};

#[allow(unused)]
async fn get_file_info() {
    let request = oss::Request::new()
        .access_key_id("LTAI5tCpYAHHsoasDTH7hfXW")
        .access_key_secret("k0JAQGp6NURoVSDuxR7BORorlejGmj");

    let url = "https://xuetube-dev.oss-cn-shanghai.aliyuncs.com/upload/2022/05/2d3b8dc1-6955-40de-a23b-21a1389d218f.jpg";

    let resp = request.execute(url).method(Method::HEAD).send().await;
    match resp {
        Ok(oss_data) => {
            println!("{:#?}", oss_data.headers);
        }
        Err(oss_err) => {
            println!("{}", oss_err);
        }
    }

    sleep(Duration::from_secs(2));
    let url = "https://xuetube-dev.oss-cn-shanghai.aliyuncs.com/upload/2022/05/2d3b8dc1-6955-40de-a23b-21a1389d218f.jpg?objectMeta";

    let resp = request
        .execute(url)
        .method(Method::HEAD)
        .resourse("objectMeta")
        .send()
        .await;

    match resp {
        Ok(oss_data) => {
            println!("{:#?}", oss_data.headers);
        }
        Err(oss_err) => {
            println!("{}", oss_err);
        }
    }
}

#[allow(unused)]
async fn get_buckets() {
    let resp = oss::Request::new()
        .access_key_id("LTAI5tCpYAHHsoasDTH7hfXW")
        .access_key_secret("k0JAQGp6NURoVSDuxR7BORorlejGmj")
        .execute("https://oss-cn-shanghai.aliyuncs.com")
        .send()
        .await
        .unwrap();

    println!("status code: {}", resp.status);
    println!("headers: {:#?}", resp.headers);
    let data = String::from_utf8_lossy(&resp.data);
    println!("data: {}", data);
}

#[allow(unused)]
pub async fn get_regions() {
    let resp = oss::Request::new()
        .access_key_id("LTAI5tCpYAHHsoasDTH7hfXW")
        .access_key_secret("k0JAQGp6NURoVSDuxR7BORorlejGmj")
        .execute("https://oss-cn-shanghai.aliyuncs.com/?regions")
        .send()
        .await;

    match resp {
        Ok(oss_data) => {
            println!("status code: {}", oss_data.status);
            println!("headers: {:#?}", oss_data.headers);
            let data = String::from_utf8_lossy(&oss_data.data);
            println!("data: {}", data);
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}

#[allow(unused)]
async fn create_bcuket() {
    let mut headers = oss::HeaderMap::new();
    headers.insert("x-oss-resource-group-id", "bababa".parse().unwrap());
    headers.insert("x-oss-acl", "public-read".parse().unwrap());

    let config = oss::arguments::CreateBucketConfiguration {
        storage_class: oss::arguments::StorageClass::Standard,
        data_redundancy_type: None,
    };

    let data = serde_xml_rs::to_string(&config).unwrap();
    let data = r#"
    <?xml version="1.0" encoding="UTF-8"?>
    <CreateBucketConfiguration>
        <StorageClass>Standard</StorageClass>
    </CreateBucketConfiguration>"#;
    let data = oss::Bytes::from(data);

    let resp = crate::oss::Request::new()
        .access_key_id("LTAI5tCpYAHHsoasDTH7hfXW")
        .access_key_secret("k0JAQGp6NURoVSDuxR7BORorlejGmj")
        .execute("https://xuetube-t1.oss-cn-shanghai.aliyuncs.com/")
        .method(oss::Method::PUT)
        .body(data)
        .headers(headers)
        .send()
        .await;

    match resp {
        Ok(oss_data) => {
            println!("status code: {}", oss_data.status);
            println!("headers: {:#?}", oss_data.headers);
            let data = String::from_utf8_lossy(&oss_data.data);
            println!("data: {}", data);
        }
        Err(err) => {
            println!("{:#?}", err);
            println!("{}", err);
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = utils::options_from_env();
    // println!("{:#?}", options);
    // println!("{}", options.base_url());
    // println!("{}", options.root_url());
    // create_bcuket().await;

    // get_regions().await;
    // get_buckets().await;
    // get_file_stat().await;
    // get_file_info().await;
    // get_file().await;

    // oss::entities::
    let client = oss::Client::new(options);
    client
        .DescribeRegions(oss::arguments::DescribeRegionsQuery::default())
        .await;
    println!("{:#?}", client)
}
