use crate::oss::{self, entities::worm::WormConfiguration};

use self::builder::{ExtendBucketWormBuilder, InitiateBucketWormBuilder};

pub mod builder {
	use serde::{Deserialize, Serialize};

	use crate::oss;
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
				.resourse(res)
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
					self.worm_id.unwrap_or("")
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
}

#[allow(non_snake_case)]
impl<'a> oss::Client<'a> {
	/// 调用InitiateBucketWorm接口新建一条合规保留策略。
	#[allow(non_snake_case)]
	pub fn InitiateBucketWorm(&self) -> InitiateBucketWormBuilder {
		InitiateBucketWormBuilder::new(self)
	}

	/// AbortBucketWorm用于删除未锁定的合规保留策略。
	#[allow(non_snake_case)]
	pub async fn AbortBucketWorm(&self) -> oss::Result<()> {
		let res = "worm";
		let url = format!("{}?{}", self.options.base_url(), res);

		let resp = self
			.request
			.task()
			.method(oss::Method::DELETE)
			.url(&url)
			.resourse(res)
			.send()
			.await?;

		let result = oss::Data {
			status: resp.status,
			headers: resp.headers,
			data: (),
		};
		Ok(result)
	}

	/// CompleteBucketWorm用于锁定合规保留策略。
	#[allow(non_snake_case)]
	pub async fn CompleteBucketWorm(&self, worm_id: &'a str) -> oss::Result<()> {
		let res = format!("wormId={}", worm_id);
		let url = format!("{}/?{}", self.options.base_url(), res);

		let resp = self
			.request
			.task()
			.method(oss::Method::POST)
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

	/// ExtendBucketWorm用于延长已锁定的合规保留策略对应Bucket中Object的保留天数。
	#[allow(non_snake_case)]
	pub fn ExtendBucketWorm(&self) -> ExtendBucketWormBuilder {
		ExtendBucketWormBuilder::new(self)
	}

	/// GetBucketWorm用于获取指定存储空间（Bucket）的合规保留策略信息。
	#[allow(non_snake_case)]
	pub async fn GetBucketWorm(&self) -> oss::Result<WormConfiguration> {
		let res = "worm";
		let url = format!("{}/?{}", self.options.base_url(), res);

		let resp = self.request.task().url(&url).resourse(res).send().await?;

		let content = String::from_utf8_lossy(&resp.data);
		let config = quick_xml::de::from_str(&content).unwrap();

		let result = oss::Data {
			status: resp.status,
			headers: resp.headers,
			data: config,
		};
		Ok(result)
	}
}
