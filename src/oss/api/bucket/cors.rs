use crate::oss::{self, entities::cors::CORSConfiguration, Client, Method};

#[allow(unused)]
pub struct PutBucketCorsBuilder<'a> {
	client: &'a oss::Client<'a>,
	config: CORSConfiguration,
}

#[allow(unused)]
impl<'a> PutBucketCorsBuilder<'a> {
	pub fn new(client: &'a oss::Client) -> Self {
		Self {
			client,
			config: CORSConfiguration::default(),
		}
	}

	pub fn config(mut self, value: CORSConfiguration) -> Self {
		self.config = value;
		self
	}

	pub async fn send(&self) -> oss::Result<()> {
		let res = "cors";
		let url = format!("{}/?{}", self.client.options.base_url(), res);
		let content = quick_xml::se::to_string(&self.config).unwrap();
		let data = oss::Bytes::from(content);
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
			data: (),
			status: resp.status,
			headers: resp.headers,
		};
		Ok(result)
	}
}

#[allow(non_snake_case)]
impl<'a> Client<'a> {
	/// 调用PutBucketCors接口为指定的存储空间（Bucket）设置跨域资源共享CORS（Cross-Origin Resource Sharing）规则
	pub fn PutBucketCors(&self) -> PutBucketCorsBuilder {
		PutBucketCorsBuilder::new(&self)
	}

	/// GetBucketCors接口用于获取指定存储空间（Bucket）当前的跨域资源共享CORS（Cross-Origin Resource Sharing）规则。
	pub async fn GetBucketCors(&self) -> oss::Result<CORSConfiguration> {
		let res = "cors";
		let url = format!("{}/?{}", self.options.base_url(), res);

		let resp = self
			.request
			.task()
			.url(&url)
			.resourse(res)
			.send()
			.await
			.unwrap();

		let content = String::from_utf8_lossy(&resp.data);
		let config = quick_xml::de::from_str::<CORSConfiguration>(&content).unwrap();
		let result = oss::Data {
			status: resp.status,
			headers: resp.headers,
			data: config,
		};
		Ok(result)
	}

	/// DeleteBucketCors用于关闭指定存储空间（Bucket）对应的跨域资源共享CORS（Cross-Origin Resource Sharing）功能并清空所有规则
	pub async fn DeleteBucketCors(&self) -> oss::Result<()> {
		let res = "cors";
		let url = format!("{}/?{}", self.options.base_url(), res);

		let resp = self
			.request
			.task()
			.url(&url)
			.method(Method::DELETE)
			.resourse(res)
			.send()
			.await
			.unwrap();

		let result = oss::Data {
			status: resp.status,
			headers: resp.headers,
			data: (),
		};
		Ok(result)
	}
}
