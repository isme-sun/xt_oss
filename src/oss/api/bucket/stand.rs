use crate::oss::entities::bucket::{BucketInfo, LocationConstraint};
use crate::oss::{self, entities::bucket::BucketStat};

use self::builder::{ListObjectBuilder, ListObjectsV2Builder, PutBucketBuilder};

pub mod builder {
    use crate::oss::{
        self,
        entities::{
            bucket::{ListBucketResult, ListBucketResult2},
            DataRedundancyType, OssAcl, StorageClass,
        },
    };
    use reqwest::header::HeaderMap;
    use serde::{Deserialize, Serialize};
    use std::fmt;

    #[derive(Debug, Serialize, Default)]
    pub(crate) struct PutBucketConfiguration {
        #[serde(rename = "StorageClass")]
        pub(crate) storage_class: StorageClass,
        #[serde(
            rename = "data_redundancy_type",
            skip_serializing_if = "Option::is_none"
        )]
        pub(crate) data_redundancy_type: Option<DataRedundancyType>,
    }

    #[allow(unused)]
    impl PutBucketConfiguration {
        pub(crate) fn to_xml(&self) -> String {
            let content = quick_xml::se::to_string(&self).unwrap();
            format!("{}{}", oss::XML_DOCTYPE, content)
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

        pub fn delimiter(mut self, value: &'a str) -> Self {
            self.query.delimiter = Some(value);
            self
        }

        pub fn marker(mut self, value: &'a str) -> Self {
            self.query.marker = Some(value);
            self
        }

        pub fn max_keys(mut self, value: i32) -> Self {
            self.query.max_keys = Some(value);
            self
        }

        pub fn prefix(mut self, value: &'a str) -> Self {
            self.query.prefix = Some(value);
            self
        }

        pub fn encoding_type(mut self, value: &'a str) -> Self {
            self.query.encoding_type = Some(value);
            self
        }

        pub async fn send(&self) -> oss::Result<ListBucketResult> {
            let url = {
                let base_url = self.client.options.base_url();
                format!("{}?{}", base_url, self.query)
            };

            let resp = self.client.request.task().url(&url).send().await?;

            let content = String::from_utf8_lossy(&resp.data);
            let buckets = quick_xml::de::from_str(&content).unwrap();
            let result = oss::Data {
                status: resp.status,
                headers: resp.headers,
                data: buckets,
            };
            Ok(result)
        }
    }

    #[derive(Debug)]
    pub struct PutBucketBuilder<'a> {
        client: &'a oss::Client<'a>,
        name: Option<&'a str>,
        acl: Option<OssAcl>,
        group_id: Option<&'a str>,
        config: Option<PutBucketConfiguration>,
    }

    impl<'a> PutBucketBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client) -> Self {
            Self {
                client,
                name: Default::default(),
                acl: None,
                group_id: None,
                config: None,
            }
        }

        pub fn name(mut self, value: &'a str) -> Self {
            self.name = Some(value);
            self
        }

        pub fn acl(mut self, value: OssAcl) -> Self {
            self.acl = Some(value);
            self
        }

        pub fn group_id(mut self, value: &'a str) -> Self {
            self.group_id = Some(value);
            self
        }

        pub fn storage_class(mut self, value: StorageClass) -> Self {
            match self.config {
                None => {
                    self.config = Some(PutBucketConfiguration {
                        storage_class: value,
                        ..PutBucketConfiguration::default()
                    });
                }
                Some(mut config) => {
                    config.storage_class = value;
                    self.config = Some(config);
                }
            }
            self
        }

        pub fn data_redundancy_type(mut self, value: DataRedundancyType) -> Self {
            match self.config {
                None => {
                    self.config = Some(PutBucketConfiguration {
                        data_redundancy_type: Some(value),
                        ..PutBucketConfiguration::default()
                    });
                }
                Some(mut config) => {
                    config.data_redundancy_type = Some(value);
                    self.config = Some(config);
                }
            }
            self
        }

        fn headers(&self) -> HeaderMap {
            let mut headers = HeaderMap::default();
            if let Some(acl) = &self.acl {
                headers.insert("x-oss-acl", acl.to_string().parse().unwrap());
            }
            if let Some(group_id) = &self.group_id {
                headers.insert("x-oss-resource-group-id", group_id.parse().unwrap());
            }
            headers
        }

        fn config(&self) -> Option<oss::Bytes> {
            let data = if let Some(config) = &self.config {
                Some(oss::Bytes::from(config.to_xml()))
            } else {
                None
            };
            data
        }

        /// 调用PutBucket接口创建存储空间（Bucket）。
        pub async fn send(&self) -> oss::Result<()> {
            let bucket = if let Some(name) = self.name {
                name
            } else {
                self.client.options.bucket
            };
            let url = {
                format!(
                    "{}://{}.{}",
                    self.client.options.schema(),
                    bucket,
                    self.client.options.host(),
                )
            };

            let headers = self.headers();
            let builder = self
                .client
                .request
                .task()
                .url(&url)
                .method(oss::Method::PUT)
                .headers(headers);
            let builder = if let Some(data) = self.config() {
                builder.body(data)
            } else {
                builder
            };

            let resp = builder.send().await?;

            let result = oss::Data {
                status: resp.status,
                headers: resp.headers,
                data: (),
            };
            Ok(result)
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub(crate) struct ListObjectsV2Query<'a> {
        #[serde(rename = "list-type")]
        pub list_type: i32,
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
                encoding_type: Some("url"),
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

        pub fn list_type(mut self, value: i32) -> Self {
            self.query.list_type = value;
            self
        }

        pub fn delimiter(mut self, value: &'a str) -> Self {
            self.query.delimiter = Some(value);
            self
        }

        pub fn start_after(mut self, value: &'a str) -> Self {
            self.query.delimiter = Some(value);
            self
        }

        pub fn continuation_token(mut self, value: &'a str) -> Self {
            self.query.continuation_token = Some(value);
            self
        }

        pub fn max_keys(mut self, value: i32) -> Self {
            self.query.max_keys = Some(value);
            self
        }

        pub fn prefix(mut self, value: &'a str) -> Self {
            self.query.prefix = Some(value);
            self
        }

        pub fn encoding_type(mut self, value: &'a str) -> Self {
            self.query.encoding_type = Some(value);
            self
        }

        pub fn fetch_owner(mut self, value: bool) -> Self {
            self.query.fetch_owner = Some(value);
            self
        }

        pub async fn send(&self) -> oss::Result<ListBucketResult2> {
            let url = {
                let base_url = self.client.options.base_url();
                format!("{}/?{}", base_url, self.query)
            };

            let resp = self
                .client
                .request
                .task()
                .url(&url)
                .method(oss::Method::GET)
                .send()
                .await?;

            let content = String::from_utf8_lossy(&resp.data);

            let buckets: ListBucketResult2 = quick_xml::de::from_str(&content).unwrap();
            let result = oss::Data {
                status: resp.status,
                headers: resp.headers,
                data: buckets,
            };
            Ok(result)
        }
    }
}

#[allow(non_snake_case)]
impl<'a> oss::Client<'a> {
    pub fn PutBucket(&self) -> PutBucketBuilder {
        PutBucketBuilder::new(&self)
    }

    /// 调用DeleteBucket删除某个存储空间（Bucket）。
    /// - 只有Bucket的拥有者才有权限删除该Bucket。
    /// - 为了防止误删除的发生，OSS不允许删除一个非空的Bucket。
    #[allow(private_interfaces)]
    pub async fn DeleteBucket(&self) -> oss::Result<()> {
        let url = self.options.base_url();
        let resp = self
            .request
            .task()
            .url(&url)
            .method(oss::Method::DELETE)
            .send()
            .await?;

        let result = oss::Data {
            status: resp.status,
            headers: resp.headers,
            data: (),
        };
        Ok(result)
    }

    /// GetBucket (ListObjects)接口用于列举存储空间（Bucket）中所有文件（Object）的信息。
    #[allow(private_interfaces)]
    pub fn ListObjects(&self) -> ListObjectBuilder {
        ListObjectBuilder::new(&self)
    }

    // ListObjectsV2（GetBucketV2）接口用于列举存储空间（Bucket）中所有文件（Object）的信息。
    #[allow(private_interfaces)]
    pub fn ListObjectsV2(&self) -> ListObjectsV2Builder {
        ListObjectsV2Builder::new(&self)
    }

    // 调用GetBucketInfo接口查看存储空间（Bucket）的相关信息。
    pub async fn GetBucketInfo(&self) -> oss::Result<BucketInfo> {
        let query = "bucketInfo";
        let url = format!("{}/?{}", self.options.base_url(), query);

        let resp = self
            .request
            .task()
            .url(&url)
            .resourse(&query)
            .send()
            .await
            .unwrap();

        let content = String::from_utf8_lossy(&resp.data);
        let bucket_info = quick_xml::de::from_str(&content).unwrap();
        let result = oss::Data {
            status: resp.status,
            headers: resp.headers,
            data: bucket_info,
        };
        Ok(result)
    }

    /// GetBucketLocation接口用于查看存储空间（Bucket）的位置信息。
    /// 只有Bucket的拥有者才能查看Bucket的位置信息。
    pub async fn GetBucketLocation(&self) -> oss::Result<LocationConstraint> {
        let query = "location";
        let url = format!("{}/?{}", self.options.base_url(), query);

        let resp = self
            .request
            .task()
            .url(&url)
            .resourse(&query)
            .send()
            .await?;

        let content = String::from_utf8_lossy(&resp.data);
        let bucket_stat = quick_xml::de::from_str(&content).unwrap();
        let result = oss::Data {
            status: resp.status,
            headers: resp.headers,
            data: bucket_stat,
        };
        Ok(result)
    }

    pub async fn GetBucketStat(&self) -> oss::Result<BucketStat> {
        let query = "stat";
        let url = format!("{}/?{}", self.options.base_url(), query);

        let resp = self
            .request
            .task()
            .url(&url)
            .resourse(&query)
            .send()
            .await?;

        let content = String::from_utf8_lossy(&resp.data);
        let bucket_stat = quick_xml::de::from_str(&content).unwrap();
        let result = oss::Data {
            status: resp.status,
            headers: resp.headers,
            data: bucket_stat,
        };
        Ok(result)
    }
}

#[cfg(test)]
pub mod tests {
    use crate::oss::{
        api::bucket::stand::builder::PutBucketConfiguration, entities::DataRedundancyType,
    };

    #[test]
    fn put_bucket_configuration() {
        let mut config = PutBucketConfiguration::default();

        config.data_redundancy_type = Some(DataRedundancyType::LRS);
        println!("{:#?}", config);
        println!("{}", config.to_xml());
        assert_eq!(1, 1);
    }
}
