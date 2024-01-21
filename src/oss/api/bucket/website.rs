use crate::oss::entities::website::WebsiteConfiguration;
#[allow(unused)]
use crate::oss::{self, Client, Data, Method, Result};

use self::builder::PutBucketWebsiteBuilder;

pub mod builder {
	use crate::oss::{
		self,
		entities::website::{
			ErrorDocument, IndexDocument, RoutingRule, RoutingRules, WebsiteConfiguration,
		},
	};

	pub struct PutBucketWebsiteBuilder<'a> {
		client: &'a oss::Client<'a>,
		index_document: Option<IndexDocument>,
		error_documnet: Option<ErrorDocument>,
		routing_rules: Option<Vec<RoutingRule>>,
	}

	impl<'a> PutBucketWebsiteBuilder<'a> {
		pub(crate) fn new(client: &'a oss::Client) -> Self {
			Self {
				client,
				index_document: None,
				error_documnet: None,
				routing_rules: None,
			}
		}

		pub fn with_default(mut self) -> Self {
			self.index_document = Some(IndexDocument::default());
			self.error_documnet = Some(ErrorDocument::default());
			self.routing_rules = None;
			self
		}

		pub fn with_index_document(mut self, value: IndexDocument) -> Self {
			self.index_document = Some(value);
			self
		}

		pub fn with_error_document(mut self, value: ErrorDocument) -> Self {
			self.error_documnet = Some(value);
			self
		}

		#[allow(unused)]
		pub fn add_routing_rule(mut self, value: RoutingRule) -> Self {
			self
		}

		pub fn config(&self) -> String {
			let config = WebsiteConfiguration {
				index_document: self.index_document.clone(),
				error_document: self.error_documnet.clone(),
				routing_rules: if self.routing_rules.is_none() {
					None
				} else {
					let rules = RoutingRules {
						routing_rule: self.routing_rules.clone(),
					};
					Some(rules)
				},
			};
			quick_xml::se::to_string(&config).unwrap()
		}

		pub async fn send(&self) -> oss::Result<()> {
			let query = "website";
			let url = format!("{}/?{}", self.client.options.base_url(), query);

			let config = self.config();
			let data = oss::Bytes::from(config);

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
				headers: resp.headers,
				status: resp.status,
				data: (),
			};
			Ok(result)
		}
	}
}

#[allow(non_snake_case)]
impl<'a> Client<'a> {
	/// example
	///
	/// ```no_run
	/// use xt_oss::{oss, utils};
	///
	/// async fn put_bucket_website() {
	///     dotenv::dotenv().ok();
	///     let options = utils::options_from_env();
	///     let client = oss::Client::new(options);
	///
	///     let result = client.PutBucketWebsite().with_default().send().await;
	///
	///     match result {
	///         Ok(result) => {
	///             println!("{:#?}", result);
	///         }
	///         Err(message) => {
	///             println!("{:?}", message);
	///         }
	///     }
	/// }
	///
	/// #[tokio::main]
	/// async fn main() {
	///     put_bucket_website().await;
	/// }
	/// ```
	pub fn PutBucketWebsite(&self) -> PutBucketWebsiteBuilder {
		PutBucketWebsiteBuilder::new(&self)
	}

	pub async fn GetBucketWebsite(&self) -> oss::Result<WebsiteConfiguration> {
		let query = "website";
		let url = format!("{}/?{}", self.options.base_url(), query);

		let resp = self
			.request
			.task()
			.url(&url)
			.resourse(&query)
			.send()
			.await?;

		let data = String::from_utf8_lossy(&resp.data);
		let config = quick_xml::de::from_str(&data).unwrap();

		let result = oss::Data {
			headers: resp.headers,
			status: resp.status,
			data: config,
		};
		Ok(result)
	}

	pub async fn DeleteBucketWebsite(&self) -> oss::Result<()> {
		let query = "website";
		let url = format!("{}/?{}", self.options.base_url(), query);

		let resp = self
			.request
			.task()
			.url(&url)
			.method(oss::Method::DELETE)
			.resourse(&query)
			.send()
			.await?;

		let result = oss::Data {
			headers: resp.headers,
			status: resp.status,
			data: (),
		};
		Ok(result)
	}
}

#[cfg(test)]
pub mod tests {
	use crate::oss;

	use super::builder::PutBucketWebsiteBuilder;

	#[test]
	fn test_put_bucket_website_builder() {
		let client = oss::Client::new(oss::Options::default());
		let builder = PutBucketWebsiteBuilder::new(&client).with_default();
		let left = r#"<WebsiteConfiguration><IndexDocument><Suffix>index.html</Suffix><SupportSubDir>true</SupportSubDir><Type>0</Type></IndexDocument><ErrorDocument><Key>error.html</Key><HttpStatus>404</HttpStatus></ErrorDocument></WebsiteConfiguration>"#;
		let right = builder.config();
		assert_eq!(left, right);
	}
}
