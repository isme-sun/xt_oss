#[tokio::main]
async fn main() {
    println!("hello world");
}

#[cfg(test)]
pub mod tests {
    // use log::debug;

    #[allow(unused)]
    use xt_oss::{
        oss::{http, Request},
        utils,
    };

    #[allow(unused)]
    fn init_logger() {
        let _ = env_logger::builder()
            // Include all events in tests
            .filter_level(log::LevelFilter::max())
            // Ensure events are captured by `cargo test`
            .is_test(true)
            // Ignore errors initializing the logger if tests race to configure it
            .try_init();
    }

    // #[tokio::test]
    // async fn ex_oss_request_regions() {
    //     // let url = "https://oss-cn-hangzhou.aliyuncs.com/?regions=oss-us-west-1";
    //     let url = "https://oss-cn-hangzhou.aliyuncs.com/?regions";

    //     let resp = Request::new()
    //         .with_access_key_id("LTAI5tCpYAHHsoasDTH7hfXW")
    //         .with_access_key_secret("k0JAQGp6NURoVSDuxR7BORorlejGmj")
    //         .task()
    //         .with_url(&url)
    //         .with_method(http::Method::GET)
    //         .execute()
    //         .await;

    //     match resp {
    //         Ok(resp) => {
    //             println!("status: {}", resp.status);
    //             println!("headers: {:#?}", resp.headers);
    //             println!("data: {}", String::from_utf8_lossy(&resp.body));
    //         }
    //         Err(message) => {
    //             println!("{:#?}", message)
    //         }
    //     }
    // }

    // #[tokio::test]
    // async fn ex_oss_request_list_buckets() {
    //     let url = "https://oss-cn-hangzhou.aliyuncs.com";

    //     let resp = Request::new()
    //         .with_access_key_id("LTAI5tCpYAHHsoasDTH7hfXW")
    //         .with_access_key_secret("k0JAQGp6NURoVSDuxR7BORorlejGmj")
    //         .task()
    //         .with_url(&url)
    //         .with_method(http::Method::GET)
    //         .execute()
    //         .await;

    //     match resp {
    //         Ok(resp) => {
    //             println!("status: {}", resp.status);
    //             println!("headers: {:#?}", resp.headers);
    //             println!("data: {}", String::from_utf8_lossy(&resp.body));
    //         }
    //         Err(message) => {
    //             println!("{:#?}", message)
    //         }
    //     }
    // }

    #[tokio::test]
    async fn ex_oss_request_bucket_info() {
        // let url = "https://xuetube-dev.oss-cn-hangzhou.aliyuncs.com/?bucketInfo";
        let url = "https:/dev-cdn.xuetube.com/?bucketInfo";

        let resp = Request::new()
            .with_access_key_id("LTAI5tCpYAHHsoasDTH7hfXW")
            .with_access_key_secret("k0JAQGp6NURoVSDuxR7BORorlejGmj")
            .task()
            .with_url(&url)
            .with_resource("/xuetube-dev/?bucketInfo")
            .with_method(http::Method::GET)
            // .execute()
            .execute_timeout(1)
            .await;

        // println!("{:#?}", resp);

        match resp {
            Ok(resp) => {
                println!("{:#?}",resp);
                println!("{:#?}",resp.content_length());
                // println!("status: {}", resp.status);
                // println!("headers: {:#?}", resp.headers);
                // println!("data: {}", String::from_utf8_lossy(&resp.body));
            }
            Err(error) => {
                println!("{:#?}", error);
                println!("is_timeout: {}", error.is_timeout())
            }
        }
    }



    // #[tokio::test]
    // async fn t2() {
    //     dotenv::dotenv().ok();
    //     let options = utils::options_from_env();
    //     let client = oss::Client::new(options);
    //     let result = client
    //         .DescribeRegions()
    //         .with_region("oss-us-west-1")
    //         .with_timeout(34)
    //         .execute()
    //         .await;
    //     match result {
    //         Ok(result) => {
    //             println!("{:#?}", result);
    //         }
    //         Err(message) => {
    //             println!("{:#?}", message);
    //         }
    //     }
    // }

    // #[tokio::test]
    // async fn t3() {
    //     dotenv::dotenv().ok();
    //     let options = utils::options_from_env();
    //     let client = oss::Client::new(options);
    //     let result = client
    //         .ListBuckets()
    //         // .with_max_keys(2)
    //         // .with_prefix("xuetube1")
    //         // .with_resource_group_id("rg-aekzjlcn4s63s7a")
    //         // .with_marker("mybucket10")
    //         .execute()
    //         .await;

    //     match result {
    //         Ok(result) => match result.body.buckets.bucket {
    //             Some(buckes) => {
    //                 for bucket in buckes {
    //                     println!("  location: {}", bucket.location);
    //                     println!("    bucket: {}", bucket.name);
    //                     println!("created_at: {}", bucket.creation_date);
    //                     println!("   extrant: {}", bucket.extranet_endpoint);
    //                     println!("  intranet: {}", bucket.intranet_endpoint);
    //                     println!("{}", "-".repeat(60))
    //                 }
    //             }
    //             None => println!("{}", "no buckets"),
    //         },
    //         Err(message) => {
    //             println!("{:#?}", message);
    //         }
    //     }
    // }
}
