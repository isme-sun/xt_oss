use crate::oss::{
	self,
	entities::cname::{
		BucketCnameConfiguration, CertificateConfiguration, Cname, CnameToken, ListCnameResult,
	},
};

#[allow(unused)]
pub struct PutCnameBuilder<'a> {
	client: &'a oss::Client<'a>,
	bucket_cname_configuration: BucketCnameConfiguration,
}

#[allow(unused)]
impl<'a> PutCnameBuilder<'a> {
	pub(crate) fn new(client: &'a oss::Client) -> Self {
		PutCnameBuilder {
			client,
			bucket_cname_configuration: BucketCnameConfiguration {
				cname: Cname {
					certificate_configuration: Some(CertificateConfiguration::default()),
					..Cname::default()
				},
			},
		}
	}

	pub fn with_domain(mut self, value: &str) -> Self {
		self.bucket_cname_configuration.cname.domain = value.to_string();
		self
	}

	pub fn with_cert_id(mut self, value: &str) -> Self {
		let mut config = self
			.bucket_cname_configuration
			.cname
			.certificate_configuration
			.unwrap();
		config.cert_id = value.to_string();
		self.bucket_cname_configuration
			.cname
			.certificate_configuration = Some(config);
		self
	}

	pub fn with_certificate(mut self, value: &str) -> Self {
		let mut config = self
			.bucket_cname_configuration
			.cname
			.certificate_configuration
			.unwrap();
		config.certificate = value.to_string();
		self.bucket_cname_configuration
			.cname
			.certificate_configuration = Some(config);
		self
	}

	pub fn with_private_key(mut self, value: &str) -> Self {
		let mut config = self
			.bucket_cname_configuration
			.cname
			.certificate_configuration
			.unwrap();
		config.private_key = value.to_string();
		self.bucket_cname_configuration
			.cname
			.certificate_configuration = Some(config);
		self
	}

	pub fn with_previous_cert_id(mut self, value: &str) -> Self {
		let mut config = self
			.bucket_cname_configuration
			.cname
			.certificate_configuration
			.unwrap();
		config.previous_cert_id = value.to_string();
		self.bucket_cname_configuration
			.cname
			.certificate_configuration = Some(config);
		self
	}

	pub fn with_force(mut self, value: bool) -> Self {
		let mut config = self
			.bucket_cname_configuration
			.cname
			.certificate_configuration
			.unwrap();
		config.force = value;
		self.bucket_cname_configuration
			.cname
			.certificate_configuration = Some(config);
		self
	}

	pub fn with_delete_certificate(mut self, value: bool) -> Self {
		let mut config = self
			.bucket_cname_configuration
			.cname
			.certificate_configuration
			.unwrap();
		config.delete_certificate = value;
		self.bucket_cname_configuration
			.cname
			.certificate_configuration = Some(config);
		self
	}

	pub fn config(&self) -> String {
		quick_xml::se::to_string(&self.bucket_cname_configuration).unwrap()
	}

	pub async fn send(&self) -> oss::Result<()> {
		let query = "cname&comp=add";
		let url = format!("{}/?{}", self.client.options.base_url(), query);

		let config = self.config();

		println!("===");
		println!("{}", &config);
		println!("===");

		let data = oss::Bytes::from(config);
		let resp = self
			.client
			.request
			.task()
			.url(&url)
			.method(oss::Method::POST)
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
}

#[allow(non_snake_case)]
impl<'a> oss::Client<'a> {
	/// 调用CreateCnameToken接口创建域名所有权验证所需的CnameToken
	pub async fn CreateCnameToken(&self, cname: &'a str) -> oss::Result<CnameToken> {
		let query = "cname&comp=token";
		let url = format!("{}/?{}", &self.options.base_url(), query);

		let mut config = BucketCnameConfiguration::default();
		config.cname.domain = cname.to_string();
		let config_content = quick_xml::se::to_string(&config).unwrap();
		let data = oss::Bytes::from(config_content);

		let resp = self
			.request
			.task()
			.url(&url)
			.method(oss::Method::POST)
			.resourse(query)
			.body(data)
			.send()
			.await?;

		let content = String::from_utf8_lossy(&resp.data);
		let cname_token = quick_xml::de::from_str::<CnameToken>(&content).unwrap();
		let result = oss::Data {
			status: resp.status,
			headers: resp.headers,
			data: cname_token,
		};
		Ok(result)
	}

	/// 调用GetCnameToken接口获取已创建的CnameToken
	pub async fn GetCnameToken(&self, cname: &'a str) -> oss::Result<CnameToken> {
		let query = format!("cname={}&comp=token", cname);
		let url = format!("{}/?{}", self.options.base_url(), &query);

		let resp = self
			.request
			.task()
			.url(&url)
			.resourse(&query)
			.send()
			.await?;

		let content = String::from_utf8_lossy(&resp.data);
		let cname_token = quick_xml::de::from_str::<CnameToken>(&content).unwrap();
		let result = oss::Data {
			status: resp.status,
			headers: resp.headers,
			data: cname_token,
		};
		Ok(result)
	}

	/// 调用PutCname接口为某个存储空间（Bucket）绑定自定义域名
	pub fn PutCname(&self) -> PutCnameBuilder {
		PutCnameBuilder::new(self)
	}

	/// 调用ListCname接口用于查询某个存储空间（Bucket）下绑定的所有的自定义域名（Cname）列表
	pub async fn ListCname(&self) -> oss::Result<ListCnameResult> {
		let res = "cname";
		let url = {
			let base_url = self.options.base_url();
			format!("{base_url}?{res}")
		};

		let resp = self.request.task().url(&url).resourse(res).send().await?;

		let content = String::from_utf8_lossy(&resp.data);

		let cnames = quick_xml::de::from_str(&content).unwrap();
		let result = oss::Data {
			status: resp.status,
			headers: resp.headers,
			data: cnames,
		};
		Ok(result)
	}

	/// 调用DeleteCname接口删除某个存储空间（Bucket）已绑定的Cname
	pub async fn DeleteCname(&self, cname: &'a str) -> oss::Result<()> {
		let res = "cname";
		let url = {
			let base_url = &self.options.base_url();
			format!("{base_url}?{res}")
		};

		let mut config = BucketCnameConfiguration::default();
		config.cname.domain = cname.to_string();
		let data = oss::Bytes::from(quick_xml::se::to_string(&config).unwrap());

		let resp = self
			.request
			.task()
			.url(&url)
			.body(data)
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
