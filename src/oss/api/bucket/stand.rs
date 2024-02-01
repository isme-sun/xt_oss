use crate::oss::{
  self,
  entities::bucket::{BucketInfo, LocationConstraint},
};

use self::builders::{ListObjectBuilder, ListObjectsV2Builder, PutBucketBuilder};

pub mod builders {
  use crate::oss::{
    self,
    api::{self, ApiResponseFrom},
    entities::{
      bucket::{CreateBucketConfiguration, ListBucketResult, ListBucketResult2},
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
        headers.insert("x-oss-acl", acl.to_string().parse().unwrap());
      }
      if let Some(group_id) = &self.group_id {
        headers.insert("x-oss-resource-group-id", group_id.parse().unwrap());
      }
      headers
    }

    fn config(&self) -> String {
      let config = CreateBucketConfiguration {
        storage_class: self.storage_class.clone(),
        data_redundancy_type: self.data_redundancy_type.clone(),
      };
      quick_xml::se::to_string(&config).unwrap()
    }

    /// 调用PutBucket接口创建存储空间（Bucket）。
    pub async fn execute(&self) -> api::ApiResult {
      let res = format!("/{}/", self.bucket.unwrap_or(self.client.bucket()),);
      let url = format!(
        "{}://{}.{}",
        self.client.options.schema(),
        self.bucket.unwrap_or(self.client.options.bucket),
        format!(
          "{}{}.{}",
          self.region.unwrap_or(self.client.options.region),
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

  #[allow(unused)]
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
      // let url = {
      //   let base_url = self.client.options.base_url();
      //   format!("{}?{}", base_url, self.query)
      // };

      // let resp = self.client.request.task().url(&url).send().await?;

      // let content = String::from_utf8_lossy(&resp.data);
      // let buckets = quick_xml::de::from_str(&content).unwrap();
      // let result = oss::Data {
      //   status: resp.status,
      //   headers: resp.headers,
      //   data: buckets,
      // };
      // Ok(result)
      todo!()
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

    pub async fn execute(&self) -> api::ApiResult<ListBucketResult2> {
      let url = {
        let base_url = self.client.options.base_url();
        format!("{}/?{}", base_url, self.query)
      };

      let resp = self
        .client
        .request
        .task()
        .with_url(&url)
        .with_method(http::Method::GET)
        .execute()
        .await?;

      Ok(ApiResponseFrom(resp).as_type().await)

      // let content = String::from_utf8_lossy(&resp.data);

      // let buckets: ListBucketResult2 = quick_xml::de::from_str(&content).unwrap();
      // let result = oss::Data {
      //   status: resp.status,
      //   headers: resp.headers,
      //   data: buckets,
      // };
      // Ok(result)
    }
  }
}

#[allow(non_snake_case)]
impl<'a> oss::Client<'a> {
  pub fn PutBucket(&self) -> PutBucketBuilder {
    PutBucketBuilder::new(self)
  }

  /// 调用DeleteBucket删除某个存储空间（Bucket）。
  /// - 只有Bucket的拥有者才有权限删除该Bucket。
  /// - 为了防止误删除的发生，OSS不允许删除一个非空的Bucket。
  #[allow(private_interfaces)]
  #[allow(unused)]
  pub async fn DeleteBucket(&self, bucket: &'a str) -> oss::Result<()> {
    todo!()
    // let url = self.options.base_url();
    // let resp = self
    //   .request
    //   .task()
    //   .url(&url)
    //   .method(oss::Method::DELETE)
    //   .send()
    //   .await?;

    // let result = oss::Data {
    //   status: resp.status,
    //   headers: resp.headers,
    //   data: (),
    // };
    // Ok(result)
  }

  /// GetBucket (ListObjects)接口用于列举存储空间（Bucket）中所有文件（Object）的信息。
  #[allow(private_interfaces)]
  pub fn ListObjects(&self) -> ListObjectBuilder {
    ListObjectBuilder::new(self)
  }

  // ListObjectsV2（GetBucketV2）接口用于列举存储空间（Bucket）中所有文件（Object）的信息。
  #[allow(private_interfaces)]
  pub fn ListObjectsV2(&self) -> ListObjectsV2Builder {
    ListObjectsV2Builder::new(self)
  }

  // 调用GetBucketInfo接口查看存储空间（Bucket）的相关信息。
  pub async fn GetBucketInfo(&self) -> oss::Result<BucketInfo> {
    todo!()
    // let query = "bucketInfo";
    // let url = format!("{}/?{}", self.options.base_url(), query);

    // let resp = self
    //   .request
    //   .task()
    //   .url(&url)
    //   .resourse(query)
    //   .send()
    //   .await
    //   .unwrap();

    // let content = String::from_utf8_lossy(&resp.data);
    // let bucket_info = quick_xml::de::from_str(&content).unwrap();
    // let result = oss::Data {
    //   status: resp.status,
    //   headers: resp.headers,
    //   data: bucket_info,
    // };
    // Ok(result)
  }

  /// GetBucketLocation接口用于查看存储空间（Bucket）的位置信息。
  /// 只有Bucket的拥有者才能查看Bucket的位置信息。
  pub async fn GetBucketLocation(&self) -> oss::Result<LocationConstraint> {
    todo!()
    //   let query = "location";
    //   let url = format!("{}/?{}", self.options.base_url(), query);

    //   let resp = self.request.task().url(&url).resourse(query).send().await?;

    //   let content = String::from_utf8_lossy(&resp.data);
    //   let bucket_stat = quick_xml::de::from_str(&content).unwrap();
    //   let result = oss::Data {
    //     status: resp.status,
    //     headers: resp.headers,
    //     data: bucket_stat,
    //   };
    //   Ok(result)
    // }

    // pub async fn GetBucketStat(&self) -> oss::Result<BucketStat> {
    //   let query = "stat";
    //   let url = format!("{}/?{}", self.options.base_url(), query);

    //   let resp = self.request.task().url(&url).resourse(query).send().await?;

    //   let content = String::from_utf8_lossy(&resp.data);
    //   let bucket_stat = quick_xml::de::from_str(&content).unwrap();
    //   let result = oss::Data {
    //     status: resp.status,
    //     headers: resp.headers,
    //     data: bucket_stat,
    //   };
    //   Ok(result)
  }
}
