#[allow(unused)]
use xt_oss::oss::{http, Request};
use xt_oss::{oss, utils};

#[allow(unused)]
async fn t1() {
    let request = Request::new()
        .with_access_key_id("LTAI5tCpYAHHsoasDTH7hfXW")
        .with_access_key_secret("k0JAQGp6NURoVSDuxR7BORorlejGmj");
    // let resp = request
    //     .task()
    //     .with_url("https://oss-cn-hangzhou.aliyuncs.com/?regions")
    //     .with_method(http::Method::GET)
    //     .execute()
    //     .await;

    // match resp {
    //     Ok(resp) => {
    //         println!("status: {}", resp.status);
    //         println!("headers: {:#?}", resp.headers);
    //         println!("data: {}", String::from_utf8_lossy(&resp.data));
    //     }
    //     Err(message) => {
    //         println!("{:#?}", message)
    //     }
    // }

    // 列出区域与查询buckets

    let resp = request
        .task()
        // .with_url("https://aliyuncs.com?regions")
        // .with_url("https://oss-cn-hangzhou.aliyuncs.com?regions")
        // .with_url("https://oss-cn-hangzhou.aliyuncs.com?regions=oss-cn-hangzhou")
        // .with_url("https://xuetube-dev.oss-cn-shanghai.aliyuncs.com?regions")
        .with_url("https://xuetube-dev.oss-cn-shanghai.aliyuncs.com?max-keys=2&prefix=course/video")
        // .with_url("https://dev-cdn.xuetube.com")
        // .with_url("https://dev-cdn.xuetube.com?prefix=course/video&max-keys=2")
        // .with_url("https://xuetube-dev.oss-cn-shanghai.aliyuncs.com/?list-type=2&max-keys=2&prefix=course/video")
        // .with_url("https://xuetube-dev.oss-cn-shanghai.aliyuncs.com?max-keys=2&prefix=course/video")
        // .with_url("https://dev-cdn.xuetube.com?list-type=2")
        // .with_url("https://dev-cdn.xuetube.com?list-type=2&prefix=course/video&max-keys=2")
        // .with_resource("/")
        .with_resource("/xuetube-dev/")
        .execute()
        .await;

    match resp {
        Ok(resp) => {
            println!("status: {}", resp.status);
            println!("headers: {:#?}", resp.headers);
            println!("data: {}", String::from_utf8_lossy(&resp.body));
        }
        Err(message) => {
            println!("{:#?}", message)
        }
    }

    // println!("{:#?}", request);
}

#[allow(unused)]
async fn t2() {
    dotenv::dotenv().ok();
    let options = utils::options_from_env();
    let client = oss::Client::new(options);
    let result = client
        .DescribeRegions()
        .with_region("oss-us-west-1")
        .execute()
        .await;
    match result {
        Ok(result) => {
            println!("{:#?}", result);
        }
        Err(message) => {
            println!("{:#?}", message);
        }
    }
}

#[allow(unused)]
async fn t3() {
    dotenv::dotenv().ok();
    let options = utils::options_from_env();
    let client = oss::Client::new(options);
    let result = client
        .ListBuckets()
        // .with_max_keys(2)
        // .with_prefix("xuetube")
        .with_resource_group_id("rg-aekzjlcn4s63s7a")
        // .with_marker("mybucket10")
        .execute()
        .await;
    match result {
        Ok(result) => {
            println!("{:#?}", result);
        }
        Err(message) => {
            println!("{:#?}", message);
        }
    }
}

async fn t4() {
    let request = Request::new()
        .with_access_key_id("LTAI5tCpYAHHsoasDTH7hfXW")
        .with_access_key_secret("k0JAQGp6NURoVSDuxR7BORorlejGmj");

    let result = request
        .task()
        .with_url("https://xuetube-dev.oss-cn-shanghai.aliyuncs.com?max-keys=2")
        .with_resource("/xuetube-dev/")
        .with_timeout(60)
        .execute()
        .await;
    match result {
        Ok(resp) => {
            println!("{}", String::from_utf8_lossy(&resp.body));
        },
        Err(message) => {
            println!("{:#?}", message)
        }
    }
}

#[tokio::main]
async fn main() {
    t4().await;
}
