#[tokio::main]
async fn main() {
    println!("hello world");
}

#[cfg(test)]
pub mod tests {
    // use log::debug;
    #[allow(unused)]
    use xt_oss::{
        oss::{self, http, Request},
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

    #[tokio::test]
    async fn ex_oss_request_regions() {
        let url = "https://oss-cn-hangzhou.aliyuncs.com/?regions=oss-us-west-1";
        // let url = "https://oss-cn-hangzhou.aliyuncs.com/?regions";

        let resp = Request::new()
            .with_access_key_id("LTAI5tCpYAHHsoasDTH7hfXW")
            .with_access_key_secret("k0JAQGp6NURoVSDuxR7BORorlejGmj")
            .task()
            .with_url(&url)
            // default Method::GET
            // .with_method(http::Method::GET)
            .execute_timeout(30)
            // default timeout = 60
            // .execute()
            .await;

        match resp {
            Ok(resp) => {
                let bytes = resp.bytes().await.unwrap();
                let content = String::from_utf8_lossy(&bytes);
                println!("{}", content);
            }
            Err(error) => {
                println!("reqwest error: {}", error)
            }
        }
    }

    #[tokio::test]
    async fn ex_oss_request_list_buckets() {
        let url = "https://oss-cn-hangzhou.aliyuncs.com";

        let resp = Request::new()
            .with_access_key_id("LTAI5tCpYAHHsoasDTH7hfXW")
            .with_access_key_secret("k0JAQGp6NURoVSDuxR7BORorlejGmj")
            .task()
            .with_url(&url)
            // default Method::GET
            // .with_method(http::Method::GET)
            .execute()
            .await;

        match resp {
            Ok(resp) => {
                let bytes = resp.bytes().await.unwrap();
                let content = String::from_utf8_lossy(&bytes);
                println!("{}", content);
            }
            Err(error) => {
                println!("reqwest error: {}", error)
            }
        }
    }

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
            .execute_timeout(30)
            .await;

        match resp {
            Ok(resp) => {
                let bytes = resp.bytes().await.unwrap();
                let content = String::from_utf8_lossy(&bytes);
                println!("{}", content);
            }
            Err(error) => {
                println!("{:#?}", error);
            }
        }
    }

    #[tokio::test]
    async fn ex_oss_request_bucket_get_cors() {
        let url = "https://xtoss-t1.oss-cn-shanghai.aliyuncs.com/?cors";
        let resp = Request::new()
            .with_access_key_id("LTAI5tCpYAHHsoasDTH7hfXW")
            .with_access_key_secret("k0JAQGp6NURoVSDuxR7BORorlejGmj")
            .task()
            .with_url(&url)
            .with_resource("/xtoss-t1/?cors")
            .execute_timeout(30)
            .await;

        match resp {
            Ok(resp) => {
                println!("is success: {}", resp.status().is_success());
                let status = resp.status();
                let bytes = resp.bytes().await.unwrap();
                let content = String::from_utf8_lossy(&bytes);
                println!("{}", status);
                println!("{}", content);
            }
            Err(error) => {
                println!("{:#?}", error);
            }
        }
    }

    #[tokio::test]
    async fn ex_oss_request_bucket_put_cors() {
        let url = "https://xtoss-t1.oss-cn-shanghai.aliyuncs.com/?cors";

        let cors_config = r#"<?xml version="1.0" encoding="UTF-8"?>
<CORSConfiguration>
    <CORSRule>
        <AllowedOrigin>*</AllowedOrigin>
        <AllowedMethod>PUT</AllowedMethod>
        <AllowedMethod>GET</AllowedMethod>
        <AllowedHeader>Authorization</AllowedHeader>
    </CORSRule>
    <CORSRule>
        <AllowedOrigin>http://example.com</AllowedOrigin>
        <AllowedOrigin>http://example.net</AllowedOrigin>
        <AllowedMethod>GET</AllowedMethod>
        <AllowedHeader> Authorization</AllowedHeader>
        <ExposeHeader>x-oss-test</ExposeHeader>
        <ExposeHeader>x-oss-test1</ExposeHeader>
        <MaxAgeSeconds>100</MaxAgeSeconds>
    </CORSRule>
    <ResponseVary>false</ResponseVary>
</CORSConfiguration >"#.to_string();

        let data = oss::Bytes::from(cors_config);

        let resp = Request::new()
            .with_access_key_id("LTAI5tCpYAHHsoasDTH7hfXW")
            .with_access_key_secret("k0JAQGp6NURoVSDuxR7BORorlejGmj")
            .task()
            .with_url(&url)
            .with_resource("/xtoss-t1/?cors")
            .with_method(http::Method::PUT)
            .with_body(data)
            .execute_timeout(30)
            .await;

        match resp {
            Ok(resp) => {
                println!("is success: {}", resp.status().is_success());
                let status = resp.status();
                let bytes = resp.bytes().await.unwrap();
                let content = String::from_utf8_lossy(&bytes);
                println!("{}", status);
                println!("{}", content);
            }
            Err(error) => {
                println!("{:#?}", error);
            }
        }
    }


}
