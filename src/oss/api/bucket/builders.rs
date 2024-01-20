use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt};

use crate::oss::{
    self,
    entities::{
        bucket::ListBucketResult,
        encryption::{ApplyServerSideEncryptionByDefault, SSEAlgorithm, ServerSideEncryptionRule},
        style::Style,
        tag::{Tag, TagSet, Tagging},
        version::{VersioningConfiguration, VersioningStatus},
        DataRedundancyType, StorageClass,
    },
};
// --------------------------------------------------------------------------
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

#[allow(unused)]
pub struct PutBucketEncryptionBuilder<'a> {
    client: &'a oss::Client<'a>,
    algorithm: SSEAlgorithm,
    data_encryption: Option<&'a str>,
    master_key_id: Option<&'a str>,
}

#[allow(unused)]
impl<'a> PutBucketEncryptionBuilder<'a> {
    pub(crate) fn new(client: &'a oss::Client) -> Self {
        Self {
            client,
            algorithm: SSEAlgorithm::default(),
            data_encryption: None,
            master_key_id: None,
        }
    }

    pub fn algorithm(mut self, value: SSEAlgorithm) -> Self {
        self.algorithm = value;
        self
    }

    pub fn data_encryption(mut self, value: &'a str) -> Self {
        self.data_encryption = Some(value);
        self
    }

    pub fn master_key_id(mut self, value: &'a str) -> Self {
        self.master_key_id = Some(value);
        self
    }

    pub async fn send(&self) -> oss::Result<()> {
        let res = "encryption";
        let url = format!("{}/?{}", self.client.options.base_url(), res);

        let mut content = ServerSideEncryptionRule {
            apply_server_side_encryption_by_default: ApplyServerSideEncryptionByDefault {
                sse_algorithm: self.algorithm,
                kms_data_encryption: if let Some(enc) = self.data_encryption {
                    Some(enc.to_string())
                } else {
                    None
                },
                kms_master_key_id: if let Some(key_id) = self.master_key_id {
                    Some(key_id.to_string())
                } else {
                    None
                },
            },
        };

        let data = oss::Bytes::from(quick_xml::se::to_string(&content).unwrap());

        let resp = self
            .client
            .request
            .task()
            .url(&url)
            .method(oss::Method::PUT)
            .resourse(res)
            .body(data)
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

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ExtendWormConfiguration {
    #[serde(rename = "RetentionPeriodInDays")]
    pub(crate) retention_period_in_days: i32,
}

impl Default for ExtendWormConfiguration {
    fn default() -> Self {
        Self {
            retention_period_in_days: 1,
        }
    }
}

pub struct InitiateBucketWormBuilder<'a> {
    client: &'a oss::Client<'a>,
    days: i32,
}

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
pub struct ExtendBucketWormBuilder<'a> {
    client: &'a oss::Client<'a>,
    worm_id: Option<&'a str>,
    days: i32,
}

#[allow(unused)]
impl<'a> ExtendBucketWormBuilder<'a> {
    pub fn new(client: &'a oss::Client) -> Self {
        Self {
            client,
            days: 1,
            worm_id: None,
        }
    }

    pub fn worm_id(mut self, value: &'a str) -> Self {
        self.worm_id = Some(value);
        self
    }

    pub fn days(mut self, value: i32) -> Self {
        self.days = value;
        self
    }

    fn config(&self) -> String {
        let config = ExtendWormConfiguration {
            retention_period_in_days: self.days,
        };
        quick_xml::se::to_string(&config).unwrap()
    }

    pub async fn send(&self) -> oss::Result<()> {
        let bucket = self.client.options.bucket;
        let res = {
            format!(
                "wormExtend&wormId={}",
                match self.worm_id {
                    Some(worm_id) => worm_id,
                    None => "",
                }
            )
        };
        let url = { format!("{}/?{}", self.client.options.base_url(), res) };
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

pub struct DeleteBucketTagsBuilder<'a> {
    client: &'a oss::Client<'a>,
    keys: Vec<&'a str>,
}

impl<'a> DeleteBucketTagsBuilder<'a> {
    pub fn new(client: &'a oss::Client) -> Self {
        Self {
            client,
            keys: Vec::new(),
        }
    }

    pub fn delete_key(mut self, value: &'a str) -> Self {
        self.keys.push(value);
        self
    }

    pub fn delete_keys(mut self, value: Vec<&'a str>) -> Self {
        self.keys.extend(value);
        self
    }

    pub async fn send(&self) -> oss::Result<()> {
        let res = "tagging";
        let query = if self.keys.len() > 0 {
            let keys = self.keys.join(",");
            format!("{}={}", res, keys)
        } else {
            format!("{}", res)
        };
        let url = { format!("{}/?{}", self.client.options.base_url(), query) };

        println!("{}", url);

        let resp = self
            .client
            .request
            .task()
            .url(&url)
            .method(oss::Method::DELETE)
            .resourse(&query)
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

pub struct PutStyleBuilder<'a> {
    client: &'a oss::Client<'a>,
    style: Style,
}

impl<'a> PutStyleBuilder<'a> {
    pub fn new(client: &'a oss::Client<'a>) -> Self {
        Self {
            client,
            style: Style::default(),
        }
    }

    pub fn name(mut self, value: &'a str) -> Self {
        self.style.name = value.to_string();
        self
    }

    pub fn content(mut self, value: &'a str) -> Self {
        self.style.content = value.to_string();
        self
    }

    pub fn category(mut self, value: &'a str) -> Self {
        self.style.category = Some(value.to_string());
        self
    }

    pub fn style(&self) -> String {
        quick_xml::se::to_string(&self.style).unwrap()
    }

    pub async fn send(&self) -> oss::Result<()> {
        let query = format!("style&styleName={}", self.style.name);
        let url = { format!("{}?{}", self.client.options.base_url(), query) };

        let data = oss::Bytes::from(self.style());
        let resp = self
            .client
            .request
            .task()
            .url(&url)
            .method(oss::Method::PUT)
            .resourse(&query)
            .body(data)
            .send()
            .await?;

        let result = oss::Data {
            data: (),
            status: resp.status,
            headers: resp.headers,
        };
        Ok(result)
    }
}

// ----------------------------------------------------------------------

pub struct PutBucketTagsBuilder<'a> {
    client: &'a oss::Client<'a>,
    tags: HashMap<&'a str, &'a str>,
}

impl<'a> PutBucketTagsBuilder<'a> {
    pub fn new(client: &'a oss::Client) -> Self {
        Self {
            client,
            tags: HashMap::new(),
        }
    }

    /// 添加tag
    pub fn add_tag(mut self, key: &'a str, value: &'a str) -> Self {
        self.tags.insert(key, value);
        self
    }

    /// 移除tag
    pub fn remove_tag(mut self, key: &'a str) -> Self {
        self.tags.remove(key);
        self
    }

    pub fn tagging(&self) -> Tagging {
        let mut tags: Vec<Tag> = Vec::new();
        for (key, value) in self.tags.clone() {
            tags.push(Tag {
                key: String::from(key),
                value: String::from(value),
            });
        }
        Tagging {
            tag_set: TagSet { tag: Some(tags) },
        }
    }

    pub fn tagging_xml(&self) -> String {
        quick_xml::se::to_string(&self.tagging()).unwrap()
    }

    pub async fn send(&self) -> oss::Result<()> {
        let res = "tagging";
        let url = format!("{}?{}", self.client.options.base_url(), res);

        let data = oss::Bytes::from(self.tagging_xml());

        let resp = self
            .client
            .request
            .task()
            .url(&url)
            .method(oss::Method::PUT)
            .resourse(res)
            .body(data)
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

pub struct PutBucketVersioningBuilder<'a> {
    client: &'a oss::Client<'a>,
    status: VersioningStatus,
}

impl<'a> PutBucketVersioningBuilder<'a> {
    pub(crate) fn new(client: &'a oss::Client) -> Self {
        Self {
            client,
            status: VersioningStatus::Enabled,
        }
    }

    pub fn status(mut self, value: VersioningStatus) -> Self {
        self.status = value;
        self
    }

    pub async fn send(&self) -> oss::Result<()> {
        let res = "versioning";
        let url = format!("{}/?{}", self.client.options.base_url(), res);

        let config = VersioningConfiguration {
            status: Some(self.status.clone()),
        };

        let data = oss::Bytes::from(quick_xml::se::to_string(&config).unwrap());

        let resp = self
            .client
            .request
            .task()
            .url(&url)
            .method(oss::Method::PUT)
            .resourse(&res)
            .body(data)
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

#[cfg(test)]
mod tests {
    use crate::oss::{
        api::bucket::builders::CreateBucketConfiguration, entities::DataRedundancyType,
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
