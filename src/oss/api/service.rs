use crate::oss::Client;

use builders::ListBucketsBuilder;

pub mod builders {
    use serde::{Deserialize, Serialize};
    use std::fmt;

    use crate::oss::{
        self,
        api::{self, insert_custom_header, ApiResponseFrom},
        entities::bucket::ListAllMyBucketsResult,
        http,
    };

    #[derive(Debug, Serialize, Deserialize, Default)]
    struct ListBucketsQuery<'a> {
        #[serde(skip_serializing_if = "Option::is_none")]
        marker: Option<&'a str>,
        #[serde(rename(serialize = "max-keys"), skip_serializing_if = "Option::is_none")]
        max_keys: Option<i32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        prefix: Option<&'a str>,
    }

    impl<'a> fmt::Display for ListBucketsQuery<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", serde_qs::to_string(&self).unwrap())
        }
    }

    #[derive(Debug)]
    pub struct ListBucketsBuilder<'a> {
        client: &'a oss::Client<'a>,
        resource_group_id: Option<&'a str>,
        query: ListBucketsQuery<'a>,
    }

    impl<'a> ListBucketsBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client) -> Self {
            Self {
                client,
                resource_group_id: None,
                query: ListBucketsQuery::default(),
            }
        }

        pub fn with_prefix(mut self, value: &'a str) -> Self {
            self.query.prefix = Some(value);
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

        pub fn with_resource_group_id(mut self, value: &'a str) -> Self {
            self.resource_group_id = Some(value);
            self
        }

        fn query(&self) -> String {
            serde_qs::to_string(&self.query).unwrap()
        }

        fn headers(&self) -> http::HeaderMap {
            let mut headers = http::HeaderMap::new();
            if let Some(group_id) = self.resource_group_id {
                insert_custom_header(&mut headers, "x-oss-resource-group-id", group_id);
            }
            headers
        }

        pub async fn execute(&self) -> api::ApiResult<ListAllMyBucketsResult> {
            let query = self.query();
            let headers = self.headers();

            let mut url = self.client.root_url();

            if !query.is_empty() {
                url = format!("{}/?{}", url, query)
            }

            let resp = self
                .client
                .request
                .task()
                .with_method(http::Method::GET)
                .with_headers(headers)
                .with_url(&url)
                .execute_timeout(self.client.timeout())
                .await?;

            Ok(ApiResponseFrom(resp).to_type().await)
        }
    }
}

#[allow(non_snake_case)]
/// 关于Region操作
impl<'a> Client<'a> {
    /// 调用ListBuckets（GetService）接口列举请求者拥有的所有存储空间（Bucket）。
    /// 您还可以通过设置prefix、marker或者max-keys参数列举满足指定条件的存储空间。
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/listbuckets)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_service_list_buckets.rs)
    pub fn ListBuckets(&self) -> ListBucketsBuilder {
        ListBucketsBuilder::new(self)
    }
}
