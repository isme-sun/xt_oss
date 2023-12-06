use std::{thread::sleep, time::Duration};

use reqwest::Method;
// use std::{thread::sleep, time::Duration};
// use xt_oss::utils::options_from_env;
use xt_oss::oss;
use xt_oss::utils;

#[allow(unused)]
async fn get_file_info() {
    let request = oss::Request::new()
        .access_key_id("LTAI5tCpYAHHsoasDTH7hfXW")
        .access_key_secret("k0JAQGp6NURoVSDuxR7BORorlejGmj");

    let url = "https://xuetube-dev.oss-cn-shanghai.aliyuncs.com/upload/2022/05/2d3b8dc1-6955-40de-a23b-21a1389d218f.jpg";

    let resp = request.task().url(url).method(Method::HEAD).send().await;
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
        .task()
        .url(url)
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
        .task()
        .url("https://oss-cn-shanghai.aliyuncs.com")
        .send()
        .await
        .unwrap();

    println!("status code: {}", resp.status);
    println!("headers: {:#?}", resp.headers);
    let data = String::from_utf8_lossy(&resp.data);
    println!("data: {}", data);
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = utils::options_from_env();
    let client = oss::Client::new(options);
    let object = client.GetObjectMeta("1.txt").await.unwrap();
    println!("{:#?}", object);

}
