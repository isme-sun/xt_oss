mod inner {
    use crate::utils::{base64_encode, get_gmt_date, hmac_sha1};
    // use base64::Engine;
    use chrono::{DateTime, Utc};
    use http::{self};
    // use serde::{Deserialize, Serialize};
    use std::fmt::Display;
    pub(super) struct AuthParams {
        pub(super) verb: http::Method,
        pub(super) date: DateTime<Utc>,
        pub(super) object_key: Option<String>,
        pub(super) bucket: Option<String>,
        pub(super) sub_res: Option<String>,
    }
    // AccelerateEndpoint
    // #[allow(non_snake_case)]
    // #[derive(Debug, Serialize, Deserialize, PartialEq)]
    // pub(super) struct PrivateRegionInfoResult {
    //     #[serde(rename = "$value")]
    //     pub(super) RegionInfoList: Vec<RegionInfo>,
    // }

    /// #### 签章计算
    ///
    ///~~~
    /// Signature = base64(hmac-sha1(AccessKeySecret,
    /// VERB + "\n"
    ///  Content-MD5 + "\n"
    ///  Content-Type + "\n"
    ///  Date + "\n"
    ///  CanonicalizedOSSHeaders
    ///  CanonicalizedResource))
    /// ~~~
    #[allow(dead_code)]
    #[derive(Debug, Default)]
    pub(super) struct Signature {
        pub(super) access_key_secret: String,
        pub(super) verb: http::Method,
        pub(super) date: DateTime<Utc>,
        pub(super) canonicalized_resource: CanonicalizedResource,
    }

    impl Display for Signature {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let value = format!(
                "{VERB}\n\napplication/octet-stream\n{Date}\n{cr}",
                VERB = &self.verb.to_string(),
                Date = get_gmt_date(&self.date),
                cr = &self.canonicalized_resource.to_string()
            );
            // println!("{:?}", value);
            let value = hmac_sha1(&self.access_key_secret.to_string(), &value.to_string());
            let encoded: String = base64_encode(value.as_slice());
            // encoded
            write!(f, "{}", encoded)
        }
    }

    ///
    /// # 构建CanonicalizedResource的方法
    ///
    /// - 发送请求中希望访问的OSS目标资源被称为CanonicalizedResource，构建方法如下：
    /// - 如果既有BucketName也有ObjectName，则CanonicalizedResource格式为/BucketName/ObjectName
    /// - 如果仅有BucketName而没有ObjectName，则CanonicalizedResource格式为/BucketName/。
    /// - 如果既没有BucketName也没有ObjectName，则CanonicalizedResource为正斜线（/）。
    /// - 如果请求的资源包括子资源（SubResource），则所有的子资源需按照字典序升序排列，并以&为分隔符生成子资源字符串。
    #[allow(dead_code)]
    #[derive(Debug, Default)]
    pub(super) struct CanonicalizedResource {
        pub(super) bucket: Option<String>,
        pub(super) object_key: Option<String>,
        pub(super) res: Option<String>,
    }

    impl Display for CanonicalizedResource {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
            if let Some(res) = &self.res {
                write!(f, "{}?{}", res_path, res)
            } else {
                write!(f, "{}", res_path)
            }
        }
    }

    #[allow(unused)]
    #[derive(Debug)]
    pub(crate) struct Authorization {
        verb: http::Method,
        date: DateTime<Utc>,
        object_key: Option<String>,
        bucket: Option<String>,
        // ! 命名
        sub_res: Option<String>,
    }

    impl Default for Authorization {
        fn default() -> Self {
            Self {
                // 请求方法
                verb: http::Method::GET,
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

    #[allow(unused)]
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

        fn to_value(&self, access_key_id: &str, key_secret: &str) -> String {
            format!("OSS {}:{}", access_key_id, self.signature(key_secret))
        }
    }
}

use crate::{
    common::{
        BucketInfo, BucketStat, ListBucketResult, ListCnameResult, OssData, OssResult,
        RegionInfoList, RegionInfo,
    },
    params::{DescribeRegionsQuery, ListObject2Query},
};

use chrono::Utc;
use http::{header, HeaderValue};

use crate::{params::ListBucketsQuery, utils::get_gmt_date};
use crate::{OssError, OssOptions};

use self::inner::{AuthParams, CanonicalizedResource, Signature};

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

    // fn common_request() {}

    // verb date object_key, query
    fn authorization(&self, params: AuthParams) -> String {
        let AuthParams {
            verb,
            date,
            object_key,
            bucket,
            sub_res,
        } = params;

        let cr = CanonicalizedResource {
            bucket,
            object_key,
            res: sub_res,
        };

        let sign = Signature {
            access_key_secret: self.options.access_key_secret.to_string(),
            verb,
            date,
            canonicalized_resource: cr,
        };

        let auth = format!(
            "OSS {}:{}",
            self.options.access_key_id.to_string(),
            sign.to_string()
        );

        auth
    }
}
// *----------------------------------------------------------------------------------
/// 关于Service操作
impl OssClient {
    /// 调用`ListBuckets（GetService）`接口列举请求者拥有的所有存储空间`（Bucket）`。您还可以通过设置
    /// `prefix`、`marker`或者`max-keys`参数列举满足指定条件的存储空间。
    #[allow(non_snake_case, unused)]
    pub async fn DescribeRegions(
        &self,
        region: DescribeRegionsQuery,
    ) -> OssResult<Vec<RegionInfo>> {
        let url = {
            let base_url = self.options.get_root_url();
            let query_str = region.to_query();
            format!("{base_url}?{query_str}")
        };
        let dt = Utc::now();
        let method = http::Method::GET;

        let params = AuthParams {
            verb: method.clone(),
            date: dt.clone(),
            object_key: None,
            bucket: None,
            sub_res: None,
        };
        let auth = self.authorization(params);

        let client = self
            ._client
            .request(method, &url)
            .header(header::DATE, get_gmt_date(&dt))
            .header(header::AUTHORIZATION, auth.to_string());

        let req = client.try_clone().unwrap().build().unwrap();

        let response = client.send().await.unwrap();

        let headers = response.headers().clone();
        let content = response.text().await.unwrap();
        if true {
            let regoins: RegionInfoList = serde_xml_rs::from_str(&content).unwrap();
            let result = OssData {
                request: req,
                // response: resp,
                data: regoins.region_info,
            };
            Ok(result)
        } else {
            let oss_error: OssError = serde_xml_rs::from_str(&content).unwrap();
            Err(oss_error)
        }
    }
}
// *----------------------------------------------------------------------------------
/// 关于Region操作
impl OssClient {
    /// 调用ListBuckets（GetService）接口列举请求者拥有的所有存储空间（Bucket）。
    /// 您还可以通过设置prefix、marker或者max-keys参数列举满足指定条件的存储空间。
    #[allow(non_snake_case)]
    pub async fn ListBuckets(
        &self,
        query: ListBucketsQuery,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // 生成uri地址
        let url = {
            let base_url = self.options.get_root_url();
            format!("{}?{}", base_url, serde_qs::to_string(&query).unwrap())
        };
        // 当前时间 时间
        let dt = Utc::now();
        // 方法
        let method = http::Method::GET;
        // auth 计算所需参数
        let params = AuthParams {
            verb: method.clone(),
            date: dt.clone(),
            object_key: None,
            bucket: None,
            sub_res: None,
        };

        let auth = self.authorization(params);

        let response = self
            ._client
            .request(method, &url)
            .header(header::DATE, get_gmt_date(&dt))
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_static("application/octet-stream"),
            )
            .header(header::AUTHORIZATION, auth.to_string())
            .send()
            .await
            .unwrap();

        println!("{:#?}", response.status());
        println!("{:#?}", response.headers());
        let _content = response.text().await.unwrap();
        println!("{}", _content);
        // let _qs = serde_qs::to_string(&query).unwrap();
        Ok(())
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
    pub async fn ListObjectsV2(&self, qurey: ListObject2Query) -> OssResult<ListBucketResult> {
        let url = {
            let base_url = self.options.get_base_url();
            let query_str = serde_qs::to_string(&qurey).unwrap();
            format!("{base_url}?{query_str}")
        };
        // println!("{}", url);
        let dt = Utc::now();
        let method = http::Method::GET;

        let params = AuthParams {
            verb: method.clone(),
            date: dt.clone(),
            object_key: None,
            bucket: Some(self.options.bucket.to_string()),
            sub_res: None,
        };

        let auth = self.authorization(params);

        let client = self
            ._client
            .request(method, &url)
            .header(header::DATE, get_gmt_date(&dt))
            .header(header::AUTHORIZATION, auth.to_string());

        let _req = client.try_clone().unwrap().build().unwrap();

        let response = client.send().await.unwrap();

        let _headers = response.headers().clone();
        let content = response.text().await.unwrap();
        if true {
            let result: ListBucketResult = serde_xml_rs::from_str(&content).unwrap();
            let data = OssData {
                request: _req,
                data: result,
            };
            Ok(data)
        } else {
            let oss_error: OssError = serde_xml_rs::from_str(&content).unwrap();
            Err(oss_error)
        }
    }

    /// 调用GetBucketInfo接口查看存储空间（Bucket）的相关信息。
    pub async fn GetBucketInfo(&self) -> OssResult<BucketInfo> {
        // url root|base|object_key
        // query
        // body
        // method,
        // sub

        let url = {
            let base_url = self.options.get_base_url();
            let query_str = "bucketInfo".to_string();
            format!("{base_url}?{query_str}")
        };
        let dt = Utc::now();
        let method = http::Method::GET;

        let params = AuthParams {
            verb: method.clone(),
            date: dt.clone(),
            object_key: None,
            bucket: Some(self.options.bucket.to_string()),
            sub_res: Some("bucketInfo".to_string()),
        };

        let auth = self.authorization(params);

        let client = self
            ._client
            .request(method, &url)
            .header(header::DATE, get_gmt_date(&dt))
            .header(header::AUTHORIZATION, auth.to_string());

        let _req = client.try_clone().unwrap().build().unwrap();

        let response = client.send().await.unwrap();

        let _headers = response.headers().clone();
        let content = response.text().await.unwrap();
        if true {
            let result: BucketInfo = serde_xml_rs::from_str(&content).unwrap();
            let data = OssData {
                request: _req,
                data: result,
            };
            Ok(data)
        } else {
            let oss_error: OssError = serde_xml_rs::from_str(&content).unwrap();
            Err(oss_error)
        }
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

        let dt = Utc::now();
        let method = http::Method::GET;

        let params = AuthParams {
            verb: method.clone(),
            date: dt.clone(),
            object_key: None,
            bucket: Some(self.options.bucket.to_string()),
            sub_res: Some("stat".to_string()),
        };
        let auth = self.authorization(params);

        let client = self
            ._client
            .request(method, &url)
            .header(header::DATE, get_gmt_date(&dt))
            .header(header::AUTHORIZATION, auth.to_string());

        let req = client.try_clone().unwrap().build().unwrap();
        // println!("{:#?}", req);

        let response = client.send().await.unwrap();

        // println!("{:#?}", response);

        // let headers = response.headers().clone();
        let content = response.text().await.unwrap();
        // println!("{}", content);
        if true {
            let stat = serde_xml_rs::from_str::<BucketStat>(&content).unwrap();
            // println!("{:#?}", stat);
            let result = OssData {
                request: req,
                // response: resp,
                data: stat,
            };
            Ok(result)
        } else {
            let oss_error: OssError = serde_xml_rs::from_str(&content).unwrap();
            Err(oss_error)
        }
        // todo!()
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
        // println!("{}", url);
        let dt = Utc::now();
        let method = http::Method::GET;

        let params = AuthParams {
            verb: method.clone(),
            date: dt.clone(),
            object_key: None,
            bucket: Some(self.options.bucket.to_string()),
            sub_res: Some("cname".to_string()),
        };

        let auth = self.authorization(params);

        let client = self
            ._client
            .request(method, &url)
            .header(header::DATE, get_gmt_date(&dt))
            .header(header::AUTHORIZATION, auth.to_string());

        let _req = client.try_clone().unwrap().build().unwrap();

        let response = client.send().await.unwrap();

        let _headers = response.headers().clone();
        let content = response.text().await.unwrap();
        if true {
            let result: ListCnameResult = serde_xml_rs::from_str(&content).unwrap();
            let data = OssData {
                request: _req,
                data: result,
            };
            Ok(data)
        } else {
            let oss_error: OssError = serde_xml_rs::from_str(&content).unwrap();
            Err(oss_error)
        }
    }

    /// 调用DeleteCname接口删除某个存储空间（Bucket）已绑定的Cname
    pub fn DeleteCname() {
        todo!()
    }
}
#[cfg(test)]
mod tests {

    use dotenv::dotenv;

    use crate::{
        params::{DescribeRegionsQuery, ListBucketsQuery},
        utils::{base64_encode, hmac_sha1},
        OssClient, OssOptions,
    };

    use super::CanonicalizedResource;

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

    #[test]
    fn canonicalized_resource() {
        env_init();
        let options = OssOptions::from_env();
        println!("{:#?}", options);
        let cr1 = CanonicalizedResource {
            bucket: Some(options.bucket.to_string()),
            object_key: None,
            res: None,
        };
        let cr2 = CanonicalizedResource {
            bucket: Some(options.bucket.to_string()),
            object_key: Some("static/admin/app.js".to_string()),
            res: None,
        };
        let cr3 = CanonicalizedResource {
            bucket: None,
            object_key: None,
            res: None,
        };

        let left = (
            "/xuetube-dev".to_string(),
            "/xuetube-dev/static/admin/app.js".to_string(),
            "/".to_string(),
        );

        let right = (cr1.to_string(), cr2.to_string(), cr3.to_string());

        assert_eq!(left, right);
    }

    #[tokio::test]
    async fn describe_regions() {
        let client = get_client();
        let region = DescribeRegionsQuery::default();
        let _ = client.DescribeRegions(region).await;
        assert_eq!(1, 1);
    }

    #[tokio::test]
    async fn list_buckets() {
        let client = get_client();
        println!("{}\n", "-".repeat(80));

        // let query = ListBucketsQuery {
        //     prefix: Some("xu".to_string()),
        //     marker: Some("xue".to_string()),
        //     max_keys: Some(2)
        // };
        let query = ListBucketsQuery::default();
        let _ = client.ListBuckets(query).await;

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
        // let client = get_client();
        // client.ListObjectsV2();
        // println!("\n{}", "-".repeat(80));
        assert_eq!(1, 1);
    }

    #[tokio::test]
    async fn get_bucket_info() {
        println!("{}\n", "-".repeat(80));
        // let client = get_client();
        // client.GetBucketInfo();
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
        // let client = get_client();
        // client.GetBucketStat();
        // println!("\n{}", "-".repeat(80));
        assert_eq!(1, 1);
    }
}
