use crate::oss::{self, entities::lifecycle::LifecycleConfiguration};
use quick_xml;

#[allow(non_snake_case)]
impl<'a> oss::Client<'a> {
	/// 定的过期时间，自动转换与规则相匹配文件（Object）的存储类型或将其删除。
	#[allow(non_snake_case)]
	pub async fn PutBucketLifecycle(&self, config: LifecycleConfiguration) -> oss::Result<()> {
		let query = "lifecycle";
		let url = format!("{}/?{}", self.options.base_url(), query);

		let config_str = quick_xml::se::to_string(&config).unwrap();
		let data = oss::Bytes::from(config_str);

		let resp = self
			.request
			.task()
			.url(&url)
			.method(oss::Method::PUT)
			.body(data)
			.resourse(query)
			.send()
			.await?;
		let result = oss::Data {
			status: resp.status,
			headers: resp.headers,
			data: (),
		};
		Ok(result)
	}

	/// 调用GetBucketLifecycle接口查看存储空间（Bucket）的生命周期规则（Lifecycle）。
	#[allow(non_snake_case)]
	pub async fn GetBucketLifecycle(&self) -> oss::Result<LifecycleConfiguration> {
		let query = "lifecycle";
		let url = format!("{}/?{}", self.options.base_url(), query);

		let resp = self.request.task().url(&url).resourse(query).send().await?;
		let config_str = String::from_utf8_lossy(&resp.data);

		println!("{}", config_str);

		let config = quick_xml::de::from_str(&config_str).unwrap();
		let result = oss::Data {
			status: resp.status,
			headers: resp.headers,
			data: config,
		};
		Ok(result)
	}

	/// DeleteBucketLifecycle接口用于删除指定存储空间（Bucket）的生命周期规则。使用DeleteBucketLifecycle
	/// 接口删除指定Bucket所有的生命周期规则后，该Bucket中的文件（Object）不会被自动删除。只有Bucket的拥有者
	/// 才能删除该Bucket的生命周期规则。
	#[allow(non_snake_case)]
	pub async fn DeleteBucketLifecycle(&self) -> oss::Result<()> {
		let query = "lifecycle";
		let url = format!("{}/?{}", self.options.base_url(), query);

		let resp = self
			.request
			.task()
			.url(&url)
			.method(oss::Method::DELETE)
			.resourse(query)
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
