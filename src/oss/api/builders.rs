use crate::oss::{
	self,
	entities::{
		bucket::ListAllMyBucketsResult,
		region::{RegionInfo, RegionInfoList},
	},
};
use serde::{Deserialize, Serialize};
use std::fmt;

// --------------------------------------------------------------------------
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

#[derive(Debug)]
pub struct ListBucketsBuilder<'a> {
	client: &'a oss::Client<'a>,
	query: ListBucketsQuery<'a>,
}

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

		let data = quick_xml::de::from_str(&data).unwrap();
		Ok(oss::Data {
			status: resp.status,
			headers: resp.headers,
			data,
		})
	}
}

// --------------------------------------------------------------------------

#[derive(Debug, Default)]
struct DescribeRegionsQuery<'a> {
	pub regions: Option<&'a str>,
}

impl<'a> fmt::Display for DescribeRegionsQuery<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if let Some(region) = &self.regions {
			write!(f, "regions={}", region)
		} else {
			write!(f, "regions")
		}
	}
}

pub struct DescribeRegionsBuilder<'a> {
	client: &'a oss::Client<'a>,
	query: DescribeRegionsQuery<'a>,
}

impl<'a> DescribeRegionsBuilder<'a> {
	pub(crate) fn new(client: &'a oss::Client) -> Self {
		Self {
			client,
			query: DescribeRegionsQuery::default(),
		}
	}

	pub fn regions(mut self, value: &'a str) -> Self {
		self.query.regions = Some(value);
		self
	}

	pub async fn send(&self) -> oss::Result<Vec<RegionInfo>> {
		let url = {
			let base_url = self.client.options.root_url();
			let query_str = self.query.to_string();
			format!("{base_url}?{query_str}")
		};

		let resp = self.client.request.task().url(&url).send().await.unwrap();

		let content = String::from_utf8_lossy(&resp.data);
		let regoins: RegionInfoList = quick_xml::de::from_str(&content).unwrap();
		let result = oss::Data {
			status: resp.status,
			headers: resp.headers,
			data: regoins.region_info,
		};
		Ok(result)
	}
}

// --------------------------------------------------------------------------------

#[cfg(test)]
mod tests {}
