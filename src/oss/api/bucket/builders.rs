use serde::{Deserialize, Serialize};
use std::fmt;

use crate::oss::{
    self,
    arguments::{DataRedundancyType, OssAcl, StorageClass},
    entities::{BucketInfo, BucketStat, ListBucketResult, LocationConstraint, Tagging},
};

// --------------------------------------------------------------------------
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ListObject2Query<'a> {
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

impl<'a> Default for ListObject2Query<'a> {
    fn default() -> Self {
        ListObject2Query {
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

impl<'a> fmt::Display for ListObject2Query<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_qs::to_string(self).unwrap())
    }
}

pub struct ListObject2Builder<'a> {
    client: &'a oss::Client<'a>,
    query: ListObject2Query<'a>,
}

impl<'a> ListObject2Builder<'a> {
    pub fn new(client: &'a oss::Client) -> Self {
        Self {
            client,
            query: ListObject2Query::default(),
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

    pub async fn send(&self) -> oss::Result<ListBucketResult> {
        let url = {
            let base_url = self.client.options.base_url();
            format!("{}?{}", base_url, self.query)
        };

        let resp = self.client.request.task().url(&url).send().await.unwrap();

        let content = String::from_utf8_lossy(&resp.data);
        let buckets: ListBucketResult = quick_xml::de::from_str(&content).unwrap();
        let result = oss::Data {
            status: resp.status,
            headers: resp.headers,
            data: buckets,
        };
        Ok(result)
    }
}

// --------------------------------------------------------------------------
#[derive(Debug, Serialize, Default)]
pub(crate) struct CreateBucketConfiguration {
    #[serde(rename = "StorageClass")]
    pub(crate) storage_class: StorageClass,
    #[serde(
        rename = "data_redundancy_type",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) data_redundancy_type: Option<DataRedundancyType>,
}

#[allow(unused)]
impl CreateBucketConfiguration {
    pub(crate) fn to_xml(&self) -> String {
        let content = quick_xml::se::to_string(&self).unwrap();
        format!("{}{}", oss::XML_DOCTYPE, content)
    }
}

#[derive(Debug)]
#[allow(unused)]
pub struct CreateBucketBuilder<'a> {
    client: &'a oss::Client<'a>,
    name: Option<&'a str>,
    acl: Option<OssAcl>,
    group_id: Option<&'a str>,
    config: Option<CreateBucketConfiguration>,
}

#[allow(unused)]
impl<'a> CreateBucketBuilder<'a> {
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
                self.config = Some(CreateBucketConfiguration {
                    storage_class: value,
                    ..CreateBucketConfiguration::default()
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
                self.config = Some(CreateBucketConfiguration {
                    data_redundancy_type: Some(value),
                    ..CreateBucketConfiguration::default()
                });
            }
            Some(mut config) => {
                config.data_redundancy_type = Some(value);
                self.config = Some(config);
            }
        }
        self
    }

    fn headers(&self) -> oss::HeaderMap {
        let mut headers = oss::HeaderMap::default();
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

// --------------------------------------------------------------------------

#[derive(Debug)]
pub struct DeleteBucketBuilder<'a> {
    client: &'a oss::Client<'a>,
    name: Option<&'a str>,
}

impl<'a> DeleteBucketBuilder<'a> {
    pub fn new(client: &'a oss::Client) -> Self {
        Self { client, name: None }
    }

    pub fn name(mut self, value: &'a str) -> Self {
        self.name = Some(value);
        self
    }

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

        let resp = self
            .client
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
}

// --------------------------------------------------------------------------
#[derive(Debug)]
#[allow(unused)]
pub struct BucketInfoBuilder<'a> {
    client: &'a oss::Client<'a>,
    name: Option<&'a str>,
}
#[allow(unused)]
impl<'a> BucketInfoBuilder<'a> {
    pub fn new(client: &'a oss::Client) -> Self {
        Self { client, name: None }
    }

    pub fn name(mut self, value: &'a str) -> Self {
        self.name = Some(value);
        self
    }
    pub async fn send(&self) -> oss::Result<BucketInfo> {
        let bucket = if let Some(name) = self.name {
            name
        } else {
            self.client.options.bucket
        };
        let res = "bucketInfo";
        let url = {
            format!(
                "{}://{}.{}?{}",
                self.client.options.schema(),
                bucket,
                self.client.options.host(),
                res
            )
        };

        let resp = self
            .client
            .request
            .task()
            .url(&url)
            .resourse(&res)
            .send()
            .await
            .unwrap();

        let content = String::from_utf8_lossy(&resp.data);
        let bucket_info: BucketInfo = quick_xml::de::from_str(&content).unwrap();
        let result = oss::Data {
            status: resp.status,
            headers: resp.headers,
            data: bucket_info,
        };
        Ok(result)
    }
}

// --------------------------------------------------------------------------
#[derive(Debug)]
pub struct BucketStatBuilder<'a> {
    client: &'a oss::Client<'a>,
    name: Option<&'a str>,
}
#[allow(unused)]
impl<'a> BucketStatBuilder<'a> {
    pub fn new(client: &'a oss::Client) -> Self {
        Self { client, name: None }
    }

    pub fn name(mut self, value: &'a str) -> Self {
        self.name = Some(value);
        self
    }

    pub async fn send(&self) -> oss::Result<BucketStat> {
        let bucket = if let Some(name) = self.name {
            name
        } else {
            self.client.options.bucket
        };
        let res = "stat";
        let url = {
            format!(
                "{}://{}.{}?{}",
                self.client.options.schema(),
                bucket,
                self.client.options.host(),
                res
            )
        };

        let resp = self
            .client
            .request
            .task()
            .url(&url)
            .resourse(&res)
            .send()
            .await?;

        let content = String::from_utf8_lossy(&resp.data);
        let bucket_stat: BucketStat = quick_xml::de::from_str(&content).unwrap();
        let result = oss::Data {
            status: resp.status,
            headers: resp.headers,
            data: bucket_stat,
        };
        Ok(result)
    }
}

// --------------------------------------------------------------------------
#[derive(Debug)]
pub struct BucketLocationBuilder<'a> {
    client: &'a oss::Client<'a>,
    name: Option<&'a str>,
}
#[allow(unused)]
impl<'a> BucketLocationBuilder<'a> {
    pub fn new(client: &'a oss::Client) -> Self {
        Self { client, name: None }
    }

    pub fn name(mut self, value: &'a str) -> Self {
        self.name = Some(value);
        self
    }

    pub async fn send(&self) -> oss::Result<LocationConstraint> {
        let bucket = if let Some(name) = self.name {
            name
        } else {
            self.client.options.bucket
        };
        let res = "location";
        let url = {
            format!(
                "{}://{}.{}?{}",
                self.client.options.schema(),
                bucket,
                self.client.options.host(),
                res
            )
        };

        let resp = self
            .client
            .request
            .task()
            .url(&url)
            .resourse(&res)
            .send()
            .await?;

        let content = String::from_utf8_lossy(&resp.data);
        let bucket_stat: LocationConstraint = quick_xml::de::from_str(&content).unwrap();
        let result = oss::Data {
            status: resp.status,
            headers: resp.headers,
            data: bucket_stat,
        };
        Ok(result)
    }
}

// --------------------------------------------------------------------------
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct InitiateWormConfiguration {
    #[serde(rename = "RetentionPeriodInDays")]
    retention_period_in_days: i32,
}

impl Default for InitiateWormConfiguration {
    fn default() -> Self {
        Self {
            retention_period_in_days: 1,
        }
    }
}

#[allow(unused)]
pub struct InitiateBucketWormBuilder<'a> {
    client: &'a oss::Client<'a>,
    days: i32,
}

#[allow(unused)]
impl<'a> InitiateBucketWormBuilder<'a> {
    pub fn new(client: &'a oss::Client) -> Self {
        Self { client, days: 1 }
    }

    pub fn days(mut self, value: i32) -> Self {
        self.days = value;
        self
    }

    fn config(&self) -> String {
        let config = InitiateWormConfiguration {
            retention_period_in_days: self.days,
        };
        quick_xml::se::to_string(&config).unwrap()
    }

    pub async fn send(&self) -> oss::Result<()> {
        let bucket = self.client.options.bucket;
        let res = "worm";
        let url = {
            format!(
                "{}://{}.{}?{}",
                self.client.options.schema(),
                bucket,
                self.client.options.host(),
                res
            )
        };

        let config = self.config();

        let resp = self
            .client
            .request
            .task()
            .method(oss::Method::POST)
            .url(&url)
            .body(oss::Bytes::from(config))
            .resourse(&res)
            .send()
            .await?;

        let result = oss::Data {
            status: resp.status,
            headers: resp.headers,
            data: (),
        };
        Ok(result)
    }
}

#[allow(unused)]
#[derive(Debug)]
pub struct PutBucketAclBuilder<'a> {
    client: &'a oss::Client<'a>,
    acl: oss::arguments::OssAcl,
}

#[allow(unused)]
impl<'a> PutBucketAclBuilder<'a> {
    pub fn new(client: &'a oss::Client) -> Self {
        Self {
            client,
            acl: oss::arguments::OssAcl::Private,
        }
    }

    pub fn acl(mut self, value: oss::arguments::OssAcl) -> Self {
        self.acl = value;
        self
    }

    pub async fn send(&self) -> oss::Result<()> {
        let bucket = self.client.options.bucket;
        let res = "acl";
        let url = {
            format!(
                "{}://{}.{}/?{}",
                self.client.options.schema(),
                bucket,
                self.client.options.host(),
                res
            )
        };

        let mut headers = oss::HeaderMap::new();
        headers.insert("x-oss-acl", self.acl.to_string().parse().unwrap());

        let resp = self
            .client
            .request
            .task()
            .method(oss::Method::PUT)
            .headers(headers)
            .url(&url)
            .resourse(&res)
            .send()
            .await?;

        let result = oss::Data {
            status: resp.status,
            headers: resp.headers,
            data: (),
        };
        Ok(result)
    }
}

#[allow(unused)]
#[derive(Debug)]
pub struct PutBucketTagsbuilder<'a> {
    client: &'a oss::Client<'a>,
    tags: Tagging,
}

#[cfg(test)]
mod tests {
    use crate::oss::{
        api::bucket::builders::CreateBucketConfiguration, arguments::DataRedundancyType,
    };

    #[test]
    fn create_bucket_configuration() {
        let mut config = CreateBucketConfiguration::default();
        config.data_redundancy_type = Some(DataRedundancyType::LRS);
        println!("{:#?}", config);
        println!("{}", config.to_xml());
        assert_eq!(1, 1);
    }
}
