use crate::oss::{self, entities::ListAllMyBucketsResult};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Default)]
struct ListBucketsQuery<'a> {
    /// 限定此次返回Bucket的最大个数。
    prefix: Option<&'a str>,
    /// 设定结果从marker之后按字母排序的第一个开始返回。如果不设定，则从头开始返回数据。
    marker: Option<&'a str>,
    #[serde(rename = "max-keys")]
    /// 限定返回的Bucket名称必须以prefix作为前缀。如果不设定，则不过滤前缀信息。
    max_keys: Option<i32>,
}

impl<'a> fmt::Display for ListBucketsQuery<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_qs::to_string(&self).unwrap())
    }
}

#[allow(unused)]
#[derive(Debug)]
pub struct ListBucketsBuilder<'a> {
    client: &'a oss::Client<'a>,
    query: ListBucketsQuery<'a>,
}

#[allow(unused)]
impl<'a> ListBucketsBuilder<'a> {
    pub(crate) fn new(client: &'a oss::Client) -> Self {
        Self {
            client,
            query: ListBucketsQuery::default(),
        }
    }

    pub fn prefix(mut self, value: &'a str) -> Self {
        self.query.prefix = Some(value);
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

    pub async fn send(&self) -> oss::Result<ListAllMyBucketsResult> {
        let url = {
            let base_url = self.client.options.root_url();
            format!("{}?{}", base_url, self.query)
        };
        let resp = self.client.request.task().url(&url).send().await.unwrap();

        let data = String::from_utf8_lossy(&resp.data);
        let data = serde_xml_rs::from_str(&data).unwrap();
        Ok(oss::Data {
            status: resp.status,
            headers: resp.headers,
            data,
        })
    }
}

#[cfg(test)]
mod tests {}
