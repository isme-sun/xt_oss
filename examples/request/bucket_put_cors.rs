use std::env;
use xt_oss::oss::{self, http, Request};
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let access_key_id = env::var("OSS_ACCESS_KEY_ID").unwrap();
    let access_key_secret = env::var("OSS_ACCESS_KEY_SECRET").unwrap();
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
</CORSConfiguration >"#
        .to_string();

    let data = oss::Bytes::from(cors_config);

    let resp = Request::new()
        .with_access_key_id(&access_key_id)
        .with_access_key_secret(&access_key_secret)
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
