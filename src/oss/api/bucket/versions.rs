use crate::oss::entities::version::VersioningConfiguration;
#[allow(unused)]
use crate::oss::{self, Client, Data, Method, Result};

use self::builder::PutBucketVersioningBuilder;

pub mod builder {
	use crate::oss::{
		self,
		entities::version::{VersioningConfiguration, VersioningStatus},
	};

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
}

#[allow(non_snake_case)]
impl<'a> Client<'a> {
	pub fn PutBucketVersioning(&self) -> PutBucketVersioningBuilder {
		PutBucketVersioningBuilder::new(self)
	}

	pub async fn GetBucketVersioning(&self) -> oss::Result<VersioningConfiguration> {
		let res = "versioning";
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

	pub fn ListObjectVersions() {
		todo!()
	}
}
