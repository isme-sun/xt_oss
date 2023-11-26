mod inner {
    use base64::{engine::general_purpose, Engine as _};
    use chrono::{DateTime, Utc};
    use crypto::digest::Digest;
    use crypto::md5::Md5;
    use crypto::sha1::Sha1;
    use hmacsha1;

    /// 通用base64编码
    pub(super) fn base64_encode(content: &[u8]) -> String {
        let encoded = general_purpose::STANDARD.encode(content);
        encoded
    }

    /// 给出字符串的md5值
    #[allow(unused)]
    pub(super) fn md5(text: &String) -> String {
        let mut hasher = Md5::new();
        hasher.input_str(&text[..]);
        let hex = hasher.result_str();
        hex
    }

    // 计算给出字符串的sha1加密值
    #[allow(unused)]
    pub(super) fn sha1(text: &String) -> String {
        let mut hasher = Sha1::new();
        hasher.input_str(&text[..]);
        let hex = hasher.result_str();
        hex.to_string()
    }

    /// hmac sha1 计算
    ///
    /// ~~~no_run
    /// /* 转成16进制字符串 */
    /// hash.iter()
    ///     .map(|b| format!("{:02x}", b))
    ///     .collect::<Vec<String>>()
    ///     .join("")
    /// ~~~
    ///
    pub(super) fn hmac_sha1(key: &String, message: &String) -> [u8; 20] {
        let key = key.as_bytes();
        let message = message.as_bytes();
        let hash = hmacsha1::hmac_sha1(key, message);
        hash
    }

    // 获取GMT时间格式
    pub(super) fn get_gmt_date(dt: &DateTime<Utc>) -> String {
        let fmt = "%a, %d %b %Y %H:%M:%S GMT";
        dt.format(fmt).to_string()
    }

    #[derive(Debug)]
    pub(super) struct Authorization {
        pub(super) verb: reqwest::Method,
        pub(super) date: DateTime<Utc>,
        pub(super) object_key: Option<String>,
        pub(super) bucket: Option<String>,
        // ! 命名
        pub(super) sub_res: Option<String>,
    }

    impl Default for Authorization {
        fn default() -> Self {
            Self {
                // 请求方法
                verb: reqwest::Method::GET,
                // 请求时间
                date: Utc::now(),
                // 请求文件对象
                object_key: None,
                // 当前bucket
                bucket: None,
                // 资源符
                sub_res: None,
            }
        }
    }

    impl Authorization {
        pub(super) fn canonicalized_resource(&self) -> String {
            let res_path = match (&self.bucket, &self.object_key) {
                (Some(bucket), Some(object_key)) => {
                    format!("/{}/{}", bucket, object_key)
                }
                (Some(bucket), None) => {
                    format!("/{}/", bucket)
                }
                (None, None) => "/".to_string(),
                (None, Some(_)) => {
                    panic!("params error")
                }
            };
            if let Some(res) = &self.sub_res {
                format!("{}?{}", res_path, res)
            } else {
                format!("{}", res_path)
            }
        }

        pub(super) fn signature(&self, key_secret: &str) -> String {
            let value = format!(
                "{VERB}\n\napplication/octet-stream\n{Date}\n{cr}",
                VERB = &self.verb.to_string(),
                Date = get_gmt_date(&self.date),
                cr = &self.canonicalized_resource()
            );
            let value = hmac_sha1(&key_secret.to_string(), &value.to_string());
            base64_encode(value.as_slice())
        }

        pub(crate) fn to_value(&self, access_key_id: &str, key_secret: &str) -> String {
            format!("OSS {}:{}", access_key_id, self.signature(key_secret))
        }
    }
}

use self::inner::{get_gmt_date, Authorization};
use crate::common::ListAllMyBucketsResult;
use crate::params::{ListBucketsQuery, OSSQuery};
use crate::{
    common::{
        BucketInfo, BucketStat, ListBucketResult, ListCnameResult, OssData, OssError, OssOptions,
        OssResult, RegionInfo, RegionInfoList,
    },
    params::{DescribeRegionsQuery, ListObject2Query},
};

use reqwest::header;
use reqwest::header::HeaderMap;
#[allow(unused_imports)]
// use http::{header, response, HeaderValue};
use reqwest::header::HeaderValue;
use reqwest::StatusCode;

#[derive(Debug)]
#[allow(non_snake_case)]
pub struct OssClient {
    pub options: OssOptions,
    _client: reqwest::Client,
}
// *----------------------------------------------------------------------------------
/// 初始化，私有方法
impl OssClient {
    #[allow(dead_code)]
    pub fn builder(options: OssOptions) -> Self {
        let client = reqwest::Client::builder().default_headers(options.common_headers());
        OssClient {
            options,
            _client: client.build().unwrap(),
        }
    }

    async fn request(
        &self,
        url: String,
        auth: Authorization,
    ) -> Result<(StatusCode, HeaderMap, String), OssError> {
        let value = auth
            .to_value(&self.options.access_key_id, &self.options.access_key_secret)
            .to_string();
        let request = self
            ._client
            .request(auth.verb, url)
            .header(header::DATE, get_gmt_date(&auth.date))
            .header(header::AUTHORIZATION, value);

        let response = request.send().await.unwrap_or_else(|err| {
            panic!("Error: {}", err.to_string());
        });

        let status = response.status();
        let headers = response.headers().clone();
        let content = response.text().await.unwrap().to_string();

        if !status.is_success() {
            let oss_error: OssError = serde_xml_rs::from_str(&content).unwrap();
            Err(oss_error)
        } else {
            Ok((status, headers, content))
        }
    }
}
// *----------------------------------------------------------------------------------
/// 关于Service操作
impl OssClient {
    /// 调用`ListBuckets（GetService）`接口列举请求者拥有的所有存储空间`（Bucket）`。您还可以通过设置
    /// `prefix`、`marker`或者`max-keys`参数列举满足指定条件的存储空间。
    #[allow(non_snake_case)]
    pub async fn DescribeRegions(
        &self,
        region: DescribeRegionsQuery,
    ) -> OssResult<Vec<RegionInfo>> {
        let url = {
            let base_url = self.options.get_root_url();
            let query_str = region.to_query();
            format!("{base_url}?{query_str}")
        };

        let auth = Authorization::default();
        let (_status, headers, content) = self.request(url, auth).await?;

        let regoins: RegionInfoList = serde_xml_rs::from_str(&content).unwrap();
        let result = OssData {
            headers,
            data: regoins.region_info,
        };
        Ok(result)
    }
}
// *----------------------------------------------------------------------------------
/// 关于Region操作
impl OssClient {
    /// 调用ListBuckets（GetService）接口列举请求者拥有的所有存储空间（Bucket）。
    /// 您还可以通过设置prefix、marker或者max-keys参数列举满足指定条件的存储空间。
    #[allow(non_snake_case)]
    pub async fn ListBuckets(&self, query: ListBucketsQuery) -> OssResult<ListAllMyBucketsResult> {
        // 生成uri地址
        let url = {
            let base_url = self.options.get_root_url();
            format!("{}?{}", base_url, query.to_query())
        };
        let auth = Authorization::default();
        let (_status, headers, content) = self.request(url, auth).await?;
        // println!("{}", content);

        let bucket: ListAllMyBucketsResult = serde_xml_rs::from_str(&content).unwrap();
        let result = OssData {
            headers,
            data: bucket,
        };
        Ok(result)
    }
}
// *----------------------------------------------------------------------------------
/// OSS Bucket Stand
#[allow(non_snake_case)]
impl OssClient {
    /// 调用PutBucket接口创建存储空间（Bucket）。
    pub fn PutBucket(&self) {
        todo!()
    }

    /// 调用DeleteBucket删除某个存储空间（Bucket）。
    /// - 只有Bucket的拥有者才有权限删除该Bucket。
    /// - 为了防止误删除的发生，OSS不允许删除一个非空的Bucket。
    pub fn DeleteBucket(&self) {
        todo!()
    }

    /// GetBucket (ListObjects)接口用于列举存储空间（Bucket）中所有文件（Object）的信息。
    pub fn GetBucket(&self) {
        todo!()
    }

    /// ListObjectsV2（GetBucketV2）接口用于列举存储空间（Bucket）中所有文件（Object）的信息。
    #[allow(unused)]
    pub async fn ListObjectsV2(&self, qurey: ListObject2Query) -> OssResult<ListBucketResult> {
        let url = {
            let base_url = self.options.get_base_url();
            let query_str = serde_qs::to_string(&qurey).unwrap();
            format!("{base_url}?{query_str}")
        };
        let auth = Authorization {
            bucket: Some(self.options.bucket.to_string()),
            ..Authorization::default()
        };

        let (_status, headers, content) = self.request(url, auth).await?;

        let bucket: ListBucketResult = serde_xml_rs::from_str(&content).unwrap();
        let result = OssData {
            headers,
            data: bucket,
        };
        Ok(result)
    }

    /// 调用GetBucketInfo接口查看存储空间（Bucket）的相关信息。
    pub async fn GetBucketInfo(&self) -> OssResult<BucketInfo> {
        let url = {
            let base_url = self.options.get_base_url();
            let query_str = "bucketInfo".to_string();
            format!("{base_url}?{query_str}")
        };
        let auth = Authorization {
            bucket: Some(self.options.bucket.to_string()),
            sub_res: Some("bucketInfo".to_string()),
            ..Authorization::default()
        };

        let (_status, headers, content) = self.request(url, auth).await?;

        let bucket: BucketInfo = serde_xml_rs::from_str(&content).unwrap();
        let result = OssData {
            headers,
            data: bucket,
        };
        Ok(result)
    }

    /// GetBucketLocation接口用于查看存储空间（Bucket）的位置信息。
    /// 只有Bucket的拥有者才能查看Bucket的位置信息。
    pub fn GetBucketLocation(&self) {
        todo!()
    }

    pub async fn GetBucketStat(&self) -> OssResult<BucketStat> {
        let url = {
            let base_url = self.options.get_base_url();
            let query_str = "stat";
            format!("{base_url}?{query_str}")
        };
        let auth = Authorization {
            bucket: Some(self.options.bucket.to_string()),
            sub_res: Some("stat".to_string()),
            ..Authorization::default()
        };

        let (_status, headers, content) = self.request(url, auth).await?;

        let bucket: BucketStat = serde_xml_rs::from_str(&content).unwrap();
        let result = OssData {
            headers,
            data: bucket,
        };
        Ok(result)
    }
}
// *----------------------------------------------------------------------------------
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
// *----------------------------------------------------------------------------------
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
// *-----------------------------------------------------------------------------------
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
// *----------------------------------------------------------------------------------
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
// *----------------------------------------------------------------------------------
/// Bucket 自定义域名（CNAME）
#[allow(non_snake_case)]
impl OssClient {
    /// 调用CreateCnameToken接口创建域名所有权验证所需的CnameToken
    pub fn CreateCnameToken() {
        todo!()
    }

    /// 调用GetCnameToken接口获取已创建的CnameToken
    pub fn GetCnameToken() {
        todo!()
    }

    /// 调用PutCname接口为某个存储空间（Bucket）绑定自定义域名
    pub fn PutCname() {
        todo!()
    }

    /// 调用ListCname接口用于查询某个存储空间（Bucket）下绑定的所有的自定义域名（Cname）列表
    pub async fn ListCname(&self) -> OssResult<ListCnameResult> {
        let url = {
            let base_url = self.options.get_base_url();
            let query_str = "cname";
            format!("{base_url}?{query_str}")
        };
        let auth = Authorization {
            bucket: Some(self.options.bucket.to_string()),
            sub_res: Some("cname".to_string()),
            ..Authorization::default()
        };

        let (_status, headers, content) = self.request(url, auth).await?;

        let bucket: ListCnameResult = serde_xml_rs::from_str(&content).unwrap();
        let result = OssData {
            headers,
            data: bucket,
        };
        Ok(result)
    }

    /// 调用DeleteCname接口删除某个存储空间（Bucket）已绑定的Cname
    pub fn DeleteCname() {
        todo!()
    }
}
// *----------------------------------------------------------------------------------
#[cfg(test)]
mod tests {

    use crate::client::inner::*;
    use dotenv::dotenv;

    use crate::client::{OssClient, OssOptions};

    fn env_init() {
        dotenv().ok();
    }

    #[allow(unused)]
    fn get_client() -> OssClient {
        env_init();
        let options = OssOptions::from_env();
        OssClient::builder(options)
    }

    #[test]
    /// Authorization 测试
    fn authorization_compute() {
        assert_eq!(1, 0)
    }

    #[test]
    /// 签章计算过程结果测试
    fn signature_compute() {
        let sign = "GET\n\napplication/octet-stream\nThu, 23 Nov 2023 03:44:36 GMT\n/xuetube-dev/";
        let access_key = "k0JAQGp6NURoVSDuxR7BORorlejGmj";
        // let base64_value1 = "JUsvX74gVUBt18ve6LPyZol1HsE=";
        let base64_value1 = "lCeU9ruGeiBRYXDs5ch8lxsxIJA=";

        let hash_value = hmac_sha1(&access_key.to_string(), &sign.to_string());
        let hash_value = hash_value.as_slice();
        let base64_value2 = base64_encode(hash_value);

        assert_eq!(base64_value1, base64_value2);
    }
}
