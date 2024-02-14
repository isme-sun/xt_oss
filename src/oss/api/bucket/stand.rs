use crate::oss;

use self::builders::{
    DeleteBucketBuilder, GetBucketInfoBuilder, GetBucketLocationBuilder, GetBucketStatBuilder,
    ListObjectBuilder, ListObjectsV2Builder, PutBucketBuilder,
};

pub mod builders {
    use crate::oss::{
        self,
        api::{self, insert_custom_header, ApiResponseFrom},
        entities::{
            bucket::{
                BucketInfo, BucketStat, CreateBucketConfiguration, ListBucketResult,
                ListBucketResult2, LocationConstraint,
            },
            DataRedundancyType, OssAcl, StorageClass,
        },
        http,
    };
    use reqwest::header::HeaderMap;
    use serde::{Deserialize, Serialize};
    use std::fmt;

    #[derive(Debug)]
    pub struct PutBucketBuilder<'a> {
        client: &'a oss::Client<'a>,
        region: Option<&'a str>,
        bucket: Option<&'a str>,
        acl: Option<OssAcl>,
        group_id: Option<&'a str>,
        storage_class: Option<StorageClass>,
        data_redundancy_type: Option<DataRedundancyType>,
    }

    impl<'a> PutBucketBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client) -> Self {
            Self {
                client,
                region: None,
                bucket: None,
                acl: None,
                group_id: None,
                // config: None,
                storage_class: None,
                data_redundancy_type: None,
            }
        }

        pub fn with_region(mut self, value: &'a str) -> Self {
            self.region = Some(value);
            self
        }

        pub fn with_bucket(mut self, value: &'a str) -> Self {
            self.bucket = Some(value);
            self
        }

        pub fn with_acl(mut self, value: OssAcl) -> Self {
            self.acl = Some(value);
            self
        }

        pub fn with_group_id(mut self, value: &'a str) -> Self {
            self.group_id = Some(value);
            self
        }

        pub fn with_storage_class(mut self, value: StorageClass) -> Self {
            self.storage_class = Some(value);
            self
        }

        pub fn with_data_redundancy_type(mut self, value: DataRedundancyType) -> Self {
            self.data_redundancy_type = Some(value);
            self
        }

        fn headers(&self) -> HeaderMap {
            let mut headers = HeaderMap::default();
            if let Some(acl) = &self.acl {
                insert_custom_header(&mut headers, "x-oss-acl", acl.to_string());
            }
            if let Some(group_id) = &self.group_id {
                insert_custom_header(&mut headers, "x-oss-resource-group-id", group_id);
            }
            headers
        }

        fn config(&self) -> String {
            let config = CreateBucketConfiguration {
                storage_class: self.storage_class.to_owned(),
                data_redundancy_type: self.data_redundancy_type.to_owned(),
            };
            quick_xml::se::to_string(&config).unwrap()
        }

        /// 调用PutBucket接口创建存储空间（Bucket）。
        pub async fn execute(&self) -> api::ApiResult {
            let region = self.region.unwrap_or(self.client.options.region);
            let bucket = self.bucket.unwrap_or(self.client.bucket());
            let res = format!("/{}/", bucket);
            let url = format!(
                "{}://{}.{}",
                self.client.options.schema(),
                bucket,
                format!(
                    "{}{}.{}",
                    region,
                    match self.client.options.internal {
                        true => "-internal",
                        false => "",
                    },
                    oss::BASE_URL
                )
            );

            let headers = self.headers();
            let config = oss::Bytes::from(self.config());

            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_method(http::Method::PUT)
                .with_resource(&res)
                .with_headers(headers)
                .with_body(config)
                .execute()
                .await?;

            Ok(ApiResponseFrom(resp).as_empty().await)
        }
    }

    pub struct DeleteBucketBuilder<'a> {
        client: &'a oss::Client<'a>,
        region: Option<&'a str>,
        bucket: Option<&'a str>,
    }

    impl<'a> DeleteBucketBuilder<'a> {
        pub fn new(client: &'a oss::Client) -> Self {
            Self {
                client,
                region: None,
                bucket: None,
            }
        }

        pub fn with_region(mut self, region: &'a str) -> Self {
            self.region = Some(region);
            self
        }

        pub fn with_bucket(mut self, bucket: &'a str) -> Self {
            self.bucket = Some(bucket);
            self
        }

        pub async fn execute(&self) -> api::ApiResult {
            let region = self.region.unwrap_or(self.client.options.region);
            let bucket = self.bucket.unwrap_or(self.client.bucket());
            let res = format!("/{}/", bucket);
            let url = format!(
                "{}://{}.{}",
                self.client.options.schema(),
                bucket,
                format!(
                    "{}{}.{}",
                    region,
                    match self.client.options.internal {
                        true => "-internal",
                        false => "",
                    },
                    oss::BASE_URL
                )
            );

            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_resource(&res)
                .with_method(http::Method::DELETE)
                .execute()
                .await?;

            Ok(ApiResponseFrom(resp).as_empty().await)
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub(crate) struct ListObjectQuery<'a> {
        delimiter: Option<&'a str>,
        marker: Option<&'a str>,
        #[serde(rename = "max-keys")]
        max_keys: Option<i32>,
        prefix: Option<&'a str>,
        #[serde(rename = "encoding-type")]
        encoding_type: Option<&'a str>,
    }

    impl<'a> fmt::Display for ListObjectQuery<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", serde_qs::to_string(self).unwrap())
        }
    }

    impl<'a> Default for ListObjectQuery<'a> {
        fn default() -> Self {
            ListObjectQuery {
                delimiter: None,
                marker: None,
                max_keys: Some(100),
                prefix: None,
                encoding_type: None,
            }
        }
    }

    pub struct ListObjectBuilder<'a> {
        client: &'a oss::Client<'a>,
        query: ListObjectQuery<'a>,
    }

    impl<'a> ListObjectBuilder<'a> {
        pub fn new(client: &'a oss::Client) -> Self {
            Self {
                client,
                query: ListObjectQuery::default(),
            }
        }

        pub fn with_delimiter(mut self, value: &'a str) -> Self {
            self.query.delimiter = Some(value);
            self
        }

        pub fn with_marker(mut self, value: &'a str) -> Self {
            self.query.marker = Some(value);
            self
        }

        pub fn with_max_keys(mut self, value: i32) -> Self {
            self.query.max_keys = Some(value);
            self
        }

        pub fn with_prefix(mut self, value: &'a str) -> Self {
            self.query.prefix = Some(value);
            self
        }

        pub fn with_encoding_type(mut self, value: &'a str) -> Self {
            self.query.encoding_type = Some(value);
            self
        }

        pub async fn execute(&self) -> api::ApiResult<ListBucketResult> {
            let res = format!("/{}/", self.client.bucket());
            let mut url = self.client.base_url();
            let query = self.query.to_string();
            if !query.is_empty() {
                url = format!("{}?{}", url, query);
            }

            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_resource(&res)
                .execute()
                .await?;

            Ok(ApiResponseFrom(resp).as_type().await)
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub(crate) struct ListObjectsV2Query<'a> {
        #[serde(rename = "list-type")]
        pub list_type: u8,
        pub delimiter: Option<&'a str>,
        #[serde(rename = "start-after")]
        pub start_after: Option<&'a str>,
        #[serde(rename = "continuation-token")]
        pub continuation_token: Option<&'a str>,
        #[serde(rename = "max-keys")]
        pub max_keys: Option<i32>,
        pub prefix: Option<&'a str>,
        #[serde(rename = "encoding-type")]
        pub encoding_type: Option<&'a str>,
        #[serde(rename = "fetch-owner")]
        pub fetch_owner: Option<bool>,
    }

    impl<'a> Default for ListObjectsV2Query<'a> {
        fn default() -> Self {
            ListObjectsV2Query {
                list_type: 2,
                delimiter: None,
                start_after: None,
                continuation_token: None,
                max_keys: Some(100),
                prefix: None,
                encoding_type: None,
                fetch_owner: None,
            }
        }
    }

    impl<'a> fmt::Display for ListObjectsV2Query<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", serde_qs::to_string(self).unwrap())
        }
    }

    pub struct ListObjectsV2Builder<'a> {
        client: &'a oss::Client<'a>,
        query: ListObjectsV2Query<'a>,
    }

    impl<'a> ListObjectsV2Builder<'a> {
        pub fn new(client: &'a oss::Client) -> Self {
            Self {
                client,
                query: ListObjectsV2Query::default(),
            }
        }

        pub fn with_delimiter(mut self, value: &'a str) -> Self {
            self.query.delimiter = Some(value);
            self
        }

        pub fn with_start_after(mut self, value: &'a str) -> Self {
            self.query.delimiter = Some(value);
            self
        }

        pub fn with_continuation_token(mut self, value: Option<&'a str>) -> Self {
            self.query.continuation_token = value;
            self
        }

        pub fn with_max_keys(mut self, value: i32) -> Self {
            self.query.max_keys = Some(value);
            self
        }

        pub fn with_prefix(mut self, value: &'a str) -> Self {
            self.query.prefix = Some(value);
            self
        }

        pub fn with_encoding_type(mut self, value: &'a str) -> Self {
            self.query.encoding_type = Some(value);
            self
        }

        pub fn with_fetch_owner(mut self, value: bool) -> Self {
            self.query.fetch_owner = Some(value);
            self
        }

        pub async fn execute(&self) -> api::ApiResult<ListBucketResult2> {
            let mut res = format!("/{}/", self.client.bucket());
            let mut url = self.client.base_url();
            let query = self.query.to_string();
            if !query.is_empty() {
                if let Some(token) = self.query.continuation_token {
                    res = format!("{}?continuation-token={}", res, token);
                }
                url = format!("{}?{}", url, query);
            }

            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_method(http::Method::GET)
                .with_resource(&res)
                .execute()
                .await?;

            Ok(ApiResponseFrom(resp).as_type().await)
        }
    }

    pub struct GetBucketInfoBuilder<'a> {
        client: &'a oss::Client<'a>,
        bucket: Option<&'a str>,
    }

    impl<'a> GetBucketInfoBuilder<'a> {
        pub fn new(client: &'a oss::Client) -> Self {
            Self {
                client,
                bucket: None,
            }
        }

        pub fn with_bucket(mut self, value: &'a str) -> Self {
            self.bucket = Some(value);
            self
        }

        pub async fn execute(&self) -> api::ApiResult<BucketInfo> {
            let region = self.client.region();
            let bucket = self.bucket.unwrap_or(self.client.bucket());
            let res = format!("/{}/?bucketInfo", bucket);
            let url = format!(
                "{}://{}.{}?bucketInfo",
                self.client.options.schema(),
                bucket,
                format!(
                    "{}{}.{}",
                    region,
                    match self.client.options.internal {
                        true => "-internal",
                        false => "",
                    },
                    oss::BASE_URL
                )
            );

            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_resource(&res)
                .execute()
                .await?;

            Ok(ApiResponseFrom(resp).as_type().await)
        }
    }

    pub struct GetBucketLocationBuilder<'a> {
        client: &'a oss::Client<'a>,
        bucket: Option<&'a str>,
    }

    impl<'a> GetBucketLocationBuilder<'a> {
        pub fn new(client: &'a oss::Client) -> Self {
            Self {
                client,
                bucket: None,
            }
        }

        pub fn with_bucket(mut self, value: &'a str) -> Self {
            self.bucket = Some(value);
            self
        }

        pub async fn execute(&self) -> api::ApiResult<LocationConstraint> {
            let region = self.client.options.region;
            let bucket = self.bucket.unwrap_or(self.client.bucket());
            let res = format!("/{}/?location", bucket);
            let url = format!(
                "{}://{}.{}/?location",
                self.client.options.schema(),
                bucket,
                format!(
                    "{}{}.{}",
                    region,
                    match self.client.options.internal {
                        true => "-internal",
                        false => "",
                    },
                    oss::BASE_URL
                )
            );
            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_resource(&res)
                .with_method(http::Method::GET)
                .execute()
                .await?;

            Ok(ApiResponseFrom(resp).as_type().await)
        }
    }

    pub struct GetBucketStatBuilder<'a> {
        client: &'a oss::Client<'a>,
        region: Option<&'a str>,
        bucket: Option<&'a str>,
    }

    impl<'a> GetBucketStatBuilder<'a> {
        pub fn new(client: &'a oss::Client) -> Self {
            Self {
                client,
                region: None,
                bucket: None,
            }
        }

        pub fn with_region(mut self, region: &'a str) -> Self {
            self.region = Some(region);
            self
        }

        pub fn with_bucket(mut self, bucket: &'a str) -> Self {
            self.bucket = Some(bucket);
            self
        }

        pub async fn execute(&self) -> api::ApiResult<BucketStat> {
            let region = self.region.unwrap_or(self.client.options.region);
            let bucket = self.bucket.unwrap_or(self.client.bucket());
            let res = format!("/{}/?stat", bucket);
            let url = format!(
                "{}://{}.{}/?stat",
                self.client.options.schema(),
                bucket,
                format!(
                    "{}{}.{}",
                    region,
                    match self.client.options.internal {
                        true => "-internal",
                        false => "",
                    },
                    oss::BASE_URL
                )
            );

            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_resource(&res)
                .with_method(http::Method::GET)
                .execute()
                .await?;

            Ok(ApiResponseFrom(resp).as_type().await)
        }
    }
}

/// # 基础操作
#[allow(non_snake_case)]
impl<'a> oss::Client<'a> {
    /// 调用PutBucket接口创建存储空间（Bucket）。
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/putbucket)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_stand_put.rs)
    pub fn PutBucket(&self) -> PutBucketBuilder<'_> {
        PutBucketBuilder::new(self)
    }

    /// 调用DeleteBucket删除某个存储空间（Bucket）。
    ///
    /// - [official docs]()
    /// - [xtoss example]()
    pub fn DeleteBucket(&self) -> DeleteBucketBuilder<'_> {
        DeleteBucketBuilder::new(self)
    }

    /// GetBucket (ListObjects)接口用于列举存储空间（Bucket）中所有文件
    /// （Object）的信息。
    ///
    /// - [official docs]()
    /// - [xtoss example]()
    pub fn ListObjects(&self) -> ListObjectBuilder<'_> {
        ListObjectBuilder::new(self)
    }

    /// ListObjectsV2（GetBucketV2）接口用于列举存储空间（Bucket）中所有文件
    ///（Object）的信息。
    ///
    /// - [official docs]()
    /// - [xtoss example]()
    pub fn ListObjectsV2(&self) -> ListObjectsV2Builder<'_> {
        ListObjectsV2Builder::new(self)
    }

    /// 调用GetBucketInfo接口查看存储空间（Bucket）的相关信息。
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/getbucketinfo)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_stand_get_info.rs)
    pub fn GetBucketInfo(&self) -> GetBucketInfoBuilder<'_> {
        GetBucketInfoBuilder::new(self)
    }

    /// GetBucketLocation接口用于查看存储空间（Bucket）的位置信息。
    /// 只有Bucket的拥有者才能查看Bucket的位置信息。
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/getbucketlocation)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_stand_get_location.rs)
    pub fn GetBucketLocation(&self) -> GetBucketLocationBuilder<'_> {
        GetBucketLocationBuilder::new(self)
    }

    /// 调用GetBucketStat接口获取指定存储空间（Bucket）的存储容量以及文件
    /// （Object）数量
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/getbucketstat)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_stand_get_stat.rs)
    pub fn GetBucketStat(&self) -> GetBucketStatBuilder<'_> {
        GetBucketStatBuilder::new(self)
    }
}
