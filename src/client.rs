use http::header::{self, HOST};
// use std::time::Duration;

#[allow(unused_imports)]
use crate::{
    types::{RegionInfo, RegionInfoList},
    OssError, OssOptions,
};

#[derive(Debug)]
pub struct OssClient {
    pub options: OssOptions,
    inner: reqwest::Client,
}
// *-----------------------------------------------------------------------------------------------
/// 初始化，私有方法
impl OssClient {
    #[allow(dead_code)]
    pub fn builder(options: OssOptions) -> Self {
        let client = reqwest::Client::builder().build().unwrap();
        OssClient {
            options,
            inner: client,
        }
    }
}
// *-----------------------------------------------------------------------------------------------
/// 关于Service操作
impl OssClient {
    /// 调用`ListBuckets（GetService）`接口列举请求者拥有的所有存储空间`（Bucket）`。您还可以通过设置
    /// `prefix`、`marker`或者`max-keys`参数列举满足指定条件的存储空间。
    #[allow(non_snake_case)]
    pub async fn DescribeRegions(&self) -> Result<RegionInfoList, OssError> {
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
}
// *-----------------------------------------------------------------------------------------------
/// 关于Region操作
impl OssClient {
    /// 调用ListBuckets（GetService）接口列举请求者拥有的所有存储空间（Bucket）。
    /// 您还可以通过设置prefix、marker或者max-keys参数列举满足指定条件的存储空间。
    #[allow(non_snake_case)]
    pub fn ListBuckets(&self) {
        todo!()
    }
}
// *-----------------------------------------------------------------------------------------------
/// OSS Bucket Stand
impl OssClient {
    /// 调用PutBucket接口创建存储空间（Bucket）。
    #[allow(non_snake_case)]
    pub fn PutBucket(&self) {
        todo!()
    }

    /// 调用DeleteBucket删除某个存储空间（Bucket）。
    /// - 只有Bucket的拥有者才有权限删除该Bucket。
    /// - 为了防止误删除的发生，OSS不允许删除一个非空的Bucket。
    #[allow(non_snake_case)]
    pub fn DeleteBucket(&self) {
        todo!()
    }

    /// GetBucket (ListObjects)接口用于列举存储空间（Bucket）中所有文件（Object）的信息。
    #[allow(non_snake_case)]
    pub fn GetBucket(&self) {
        todo!()
    }

    /// ListObjectsV2（GetBucketV2）接口用于列举存储空间（Bucket）中所有文件（Object）的信息。
    #[allow(non_snake_case)]
    pub fn ListObjectsV2(&self) {
        todo!()
    }

    /// 调用GetBucketInfo接口查看存储空间（Bucket）的相关信息。
    #[allow(non_snake_case)]
    pub fn GetBucketInfo(&self) {
        todo!()
    }

    /// GetBucketLocation接口用于查看存储空间（Bucket）的位置信息。
    /// 只有Bucket的拥有者才能查看Bucket的位置信息。
    #[allow(non_snake_case)]
    pub fn GetBucketLocation(&self) {
        todo!()
    }

    #[allow(non_snake_case)]
    pub fn GetBucketStat(&self) {
        todo!()
    }
}
// *-----------------------------------------------------------------------------------------------
/// OSS Buckek Worm
impl OssClient {
    /// 调用InitiateBucketWorm接口新建一条合规保留策略。
    #[allow(non_snake_case)]
    pub fn InitiateBucketWorm() {
        todo!()
    }

    /// AbortBucketWorm用于删除未锁定的合规保留策略。
    #[allow(non_snake_case)]
    pub fn AbortBucketWorm() {
        todo!()
    }

    /// CompleteBucketWorm用于锁定合规保留策略。
    #[allow(non_snake_case)]
    pub fn CompleteBucketWorm() {
        todo!()
    }

    /// ExtendBucketWorm用于延长已锁定的合规保留策略对应Bucket中Object的保留天数。
    #[allow(non_snake_case)]
    pub fn ExtendBucketWorm() {
        todo!()
    }

    /// GetBucketWorm用于获取指定存储空间（Bucket）的合规保留策略信息。
    #[allow(non_snake_case)]
    pub fn GetBucketWorm() {
        todo!()
    }
}
// *-----------------------------------------------------------------------------------------------
/// OSS Bucket ACL
impl OssClient {
    /// PutBucketAcl接口用于设置或修改存储空间（Bucket）的访问权限（ACL）
    #[allow(non_snake_case)]
    pub fn PutBucketAcl() {
        todo!()
    }

    /// GetBucketAcl接口用于获取某个存储空间（Bucket）的访问权限（ACL）。只有Bucket的拥有者才能获取Bucket的访问权限。
    #[allow(non_snake_case)]
    pub fn GetBucketAcl() {
        todo!()
    }
}
// *-----------------------------------------------------------------------------------------------
/// OSS Bucket Lifecycle
impl OssClient {
    /// 调用PutBucketLifecycle接口为存储空间（Bucket）设置生命周期规则。生命周期规则开启后，OSS将按照规则中指
    /// 定的过期时间，自动转换与规则相匹配文件（Object）的存储类型或将其删除。
    #[allow(non_snake_case)]
    pub fn PutBucketLifecycle() {
        todo!()
    }

    /// 调用GetBucketLifecycle接口查看存储空间（Bucket）的生命周期规则（Lifecycle）。
    #[allow(non_snake_case)]
    pub fn GetBucketLifecycle() {
        todo!()
    }

    /// DeleteBucketLifecycle接口用于删除指定存储空间（Bucket）的生命周期规则。使用DeleteBucketLifecycle
    /// 接口删除指定Bucket所有的生命周期规则后，该Bucket中的文件（Object）不会被自动删除。只有Bucket的拥有者
    /// 才能删除该Bucket的生命周期规则。
    #[allow(non_snake_case)]
    pub fn DeleteBucketLifecycle() {
        todo!()
    }
}
// *-----------------------------------------------------------------------------------------------
/// 传输加速（TransferAcceleration）
impl OssClient {
    /// # PutBucketTransferAcceleration
    /// PutBucketTransferAcceleration接口用于为存储空间（Bucket）配置传输加速。开启传输加速后，可提升全球各
    /// 地用户对OSS的访问速度，适用于远距离数据传输、GB或TB级大文件上传和下载的场景。
    #[allow(non_snake_case)]
    pub fn PutBucketTransferAcceleration() {
        todo!()
    }

    /// GetBucketTransferAcceleration接口用于获取目标存储空间（Bucket）的传输加速配置。
    #[allow(non_snake_case)]
    pub fn GetBucketTransferAcceleration() {
        todo!()
    }
}
// *-----------------------------------------------------------------------------------------------
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
        match client.DescribeRegions().await {
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
        client.ListBuckets();
        println!("\n{}", "-".repeat(80));
        assert_eq!(1, 1);
    }

    #[tokio::test]
    async fn put_bucket() {
        println!("{}\n", "-".repeat(80));
        let client = get_client();
        client.PutBucket();
        println!("\n{}", "-".repeat(80));
        assert_eq!(1, 1);
    }

    #[tokio::test]
    async fn delete_bucket() {
        println!("{}\n", "-".repeat(80));
        let client = get_client();
        client.DeleteBucket();
        println!("\n{}", "-".repeat(80));
        assert_eq!(1, 1);
    }

    #[tokio::test]
    async fn get_bucket() {
        println!("{}\n", "-".repeat(80));
        let client = get_client();
        client.GetBucket();
        println!("\n{}", "-".repeat(80));
        assert_eq!(1, 1);
    }

    #[tokio::test]
    async fn list_objects_v2() {
        println!("{}\n", "-".repeat(80));
        let client = get_client();
        client.ListObjectsV2();
        println!("\n{}", "-".repeat(80));
        assert_eq!(1, 1);
    }

    #[tokio::test]
    async fn get_bucket_info() {
        println!("{}\n", "-".repeat(80));
        let client = get_client();
        client.GetBucketInfo();
        println!("\n{}", "-".repeat(80));
        assert_eq!(1, 1);
    }

    #[tokio::test]
    async fn get_bucket_location() {
        println!("{}\n", "-".repeat(80));
        let client = get_client();
        client.GetBucketLocation();
        println!("\n{}", "-".repeat(80));
        assert_eq!(1, 1);
    }

    #[tokio::test]
    async fn get_bucket_stat() {
        println!("{}\n", "-".repeat(80));
        let client = get_client();
        client.GetBucketStat();
        println!("\n{}", "-".repeat(80));
        assert_eq!(1, 1);
    }
}
