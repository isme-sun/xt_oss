use http::header::{self, HOST};
// use std::time::Duration;

#[allow(unused_imports)]
use crate::{
    types::{RegionInfo, RegionInfoList},
    OssError, OssOptions,
};

#[allow(unused)]
pub struct OssClient {
    pub options: OssOptions,
    inner: reqwest::Client,
}

/// OssClinet
impl OssClient {
    #[allow(dead_code)]

    pub fn builder(options: OssOptions) -> Self {
        let client = reqwest::Client::builder().build().unwrap();
        OssClient {
            options,
            inner: client,
        }
    }

    /// 调用DescribeRegions接口查询所有支持地域或者指定地域对应的Endpoint信息，
    /// 包括外网Endpoint、内网Endpoint和传输加速Endpoint。
    /// @see https://help.aliyun.com/zh/oss/developer-reference/describeregions?spm=a2c4g.11186623.0.0.371f71bf0k9bCT
    pub async fn describe_regions(&self) -> Result<RegionInfoList, OssError> {
        let oss_base_url = "aliyuncs.com";
        let default_region = "oss-cn-hangzhou";

        let url = format!("https://{}.{}/", default_region, oss_base_url);

        let mut default_headers = header::HeaderMap::new();
        default_headers.insert(HOST, oss_base_url.parse().unwrap());

        let client = self.inner.request(http::Method::GET, url);
        let response = client.send().await.unwrap();

        let content = response.text().await.unwrap();
        let oss_error: OssError = serde_xml_rs::from_str(&content).unwrap();
        Err(oss_error)
    }

    /// 调用ListBuckets（GetService）接口列举请求者拥有的所有存储空间（Bucket）。
    /// 您还可以通过设置prefix、marker或者max-keys参数列举满足指定条件的存储空间。
    pub fn list_buckets(&self) {
        todo!()
    }

    /// 调用PutBucket接口创建存储空间（Bucket）。
    pub fn put_bucket(&self) {
        todo!()
    }

    /// 调用DeleteBucket删除某个存储空间（Bucket）。
    /// - 只有Bucket的拥有者才有权限删除该Bucket。
    /// - 为了防止误删除的发生，OSS不允许删除一个非空的Bucket。
    pub fn delete_bucket(&self) {
        todo!()
    }

    /// GetBucket (ListObjects)接口用于列举存储空间（Bucket）中所有文件（Object）的信息。
    pub fn get_bucket(&self) {
        todo!()
    }

    /// ListObjectsV2（GetBucketV2）接口用于列举存储空间（Bucket）中所有文件（Object）的信息。
    pub fn list_objects_v2(&self) {
        todo!()
    }

    /// 调用GetBucketInfo接口查看存储空间（Bucket）的相关信息。
    pub fn get_bucket_info(&self) {
        todo!()
    }

    /// GetBucketLocation接口用于查看存储空间（Bucket）的位置信息。
    /// 只有Bucket的拥有者才能查看Bucket的位置信息。
    #[allow(non_snake_case)]
    pub fn get_bucket_location(&self) {
        todo!()
    }

    #[allow(non_snake_case)]
    pub fn get_bucket_stat(&self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::{OssClient, OssOptions};

    // sts_token: env::var("OSS_STS_TOKEN").unwrap_or_default(),
    // internal: env::var("OSS_INTERNAL").unwrap_or_default().parse(),
    // cname: env::var("OSS_CNAME").unwrap_or_default(),
    // is_request_pay: env::var("OSS_IS_REQUEST_PAY").unwrap_or_default(),
    // secure: env::var("OSS_SECURE").unwrap_or_default(),
    // timeout: env::var("OSS_TIMEOUT").unwrap_or_default()
    #[allow(unused)]
    fn get_options() -> OssOptions {
        dotenv::dotenv().expect("error: .env not exist");
        OssOptions {
            access_key_id: env::var("OSS_ACCESS_KEY_ID").unwrap_or_default(),
            access_key_secret: env::var("OSS_ACCESS_KEY_SECRET").unwrap_or_default(),
            bucket: env::var("OSS_BUCKET").unwrap_or_default(),
            region: env::var("OSS_REGION").unwrap_or_default(),
            ..OssOptions::default()
        }
    }

    #[allow(unused)]
    fn get_client() -> OssClient {
        OssClient::builder(get_options())
    }

    #[tokio::test]
    async fn describe_regions() {
        println!("{}\n", "-".repeat(80));
        let client = get_client();
        match client.describe_regions().await {
            Ok(regions) => {
                println!("{:?}", regions);
            }
            Err(error) => {
                println!("{:#?}", error);
            }
        };
        println!("\n{}", "-".repeat(80));
        assert_eq!(1, 1);
    }

    #[tokio::test]
    async fn list_buckets() {
        println!("{}\n", "-".repeat(80));
        let client = get_client();
        client.list_buckets();
        println!("\n{}", "-".repeat(80));
        assert_eq!(1, 1);
    }

    #[tokio::test]
    async fn put_bucket() {
        println!("{}\n", "-".repeat(80));
        let client = get_client();
        client.put_bucket();
        println!("\n{}", "-".repeat(80));
        assert_eq!(1, 1);
    }

    #[tokio::test]
    async fn delete_bucket() {
        println!("{}\n", "-".repeat(80));
        let client = get_client();
        client.delete_bucket();
        println!("\n{}", "-".repeat(80));
        assert_eq!(1, 1);
    }

    #[tokio::test]
    async fn get_bucket() {
        println!("{}\n", "-".repeat(80));
        let client = get_client();
        client.get_bucket();
        println!("\n{}", "-".repeat(80));
        assert_eq!(1, 1);
    }

    #[tokio::test]
    async fn list_objects_v2() {
        println!("{}\n", "-".repeat(80));
        let client = get_client();
        client.list_objects_v2();
        println!("\n{}", "-".repeat(80));
        assert_eq!(1, 1);
    }

    #[tokio::test]
    async fn get_bucket_info() {
        println!("{}\n", "-".repeat(80));
        let client = get_client();
        client.get_bucket_info();
        println!("\n{}", "-".repeat(80));
        assert_eq!(1, 1);
    }

    #[tokio::test]
    async fn get_bucket_location() {
        println!("{}\n", "-".repeat(80));
        let client = get_client();
        client.get_bucket_location();
        println!("\n{}", "-".repeat(80));
        assert_eq!(1, 1);
    }

    #[tokio::test]
    async fn get_bucket_stat() {
        println!("{}\n", "-".repeat(80));
        let client = get_client();
        client.get_bucket_stat();
        println!("\n{}", "-".repeat(80));
        assert_eq!(1, 1);
    }
}
