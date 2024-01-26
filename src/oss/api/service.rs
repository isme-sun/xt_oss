use crate::oss::Client;

use builder::ListBucketsBuilder;

pub mod builder {
    use serde::{Deserialize, Serialize};
    use std::fmt;

    use crate::oss::{
        self,
        api::{self, into_api_result},
        entities::bucket::ListAllMyBucketsResult,
        http,
    };

    #[derive(Debug, Serialize, Deserialize, Default)]
    struct ListBucketsQuery<'a> {
        /// 设定结果从marker之后按字母排序的第一个开始返回。如果不设定，则从头开始返回数据。
        #[serde(skip_serializing_if = "Option::is_none")]
        marker: Option<&'a str>,
        /// 限定返回的Bucket名称必须以prefix作为前缀。如果不设定，则不过滤前缀信息。
        #[serde(
            rename(serialize = "max-keys"),
            skip_serializing_if = "Option::is_none"
        )]
        max_keys: Option<i32>,
        /// 限定此次返回Bucket的最大个数。
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
        timeout: Option<u64>,
    }

    impl<'a> ListBucketsBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client) -> Self {
            Self {
                client,
                resource_group_id: None,
                timeout: None,
                query: ListBucketsQuery::default(),
            }
        }

        pub fn with_timeout(mut self, value: u64) -> Self {
            self.timeout = Some(value);
            self
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
            let headers = if let Some(group_id) = self.resource_group_id {
                headers.append("x-oss-resource-group-id", group_id.parse().unwrap());
                headers
            } else {
                headers
            };
            headers
        }

        pub async fn execute(&self) -> api::Result<ListAllMyBucketsResult> {
            let query = self.query();
            let headers = self.headers();

            let url = self.client.options.root_url();

            let url = if !query.is_empty() {
                format!("{}/?{}", url, query)
            } else {
                url
            };

            let task = self
                .client
                .request
                .task()
                .with_method(http::Method::GET)
                .with_url(&url)
                .with_resource("/");

            let task = if !headers.is_empty() {
                task.with_headers(headers)
            } else {
                task
            };

            let resp = match self.timeout {
                Some(timeout) => task.execute_timeout(timeout).await,
                None => task.execute().await,
            };

            into_api_result(resp).await
        }
    }
}

#[allow(non_snake_case)]
/// 关于Region操作
impl<'a> Client<'a> {
    // #[allow(private_interfaces)]
    pub fn ListBuckets(&self) -> ListBucketsBuilder {
        ListBucketsBuilder::new(self)
    }
}
