use crate::oss::{
	self,
	entities::style::{Style, StyleList},
	Client,
};

use self::builder::PutStyleBuilder;

pub mod builder {
	use crate::oss::{self, entities::style::Style};

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
}

#[allow(non_snake_case)]
impl<'a> Client<'a> {
	/// 调用PutStyle接口新增图片样式。一个图片样式中可以包含单个或多个图片处理参数
	pub fn PutStyle(&self) -> PutStyleBuilder {
		PutStyleBuilder::new(self)
	}

	/// 调用GetStyle接口查询某个Bucket下指定的样式信息
	pub async fn ListStyle(&self) -> oss::Result<StyleList> {
		let query = "style";
		let url = format!("{}?{}", self.options.base_url(), query);
		let resp = self
			.request
			.task()
			.url(&url)
			.method(oss::Method::GET)
			.resourse(query)
			.send()
			.await?;
		let content = String::from_utf8_lossy(&resp.data);

		let style_list: StyleList = quick_xml::de::from_str(&content).unwrap();
		let result = oss::Data {
			data: style_list,
			status: resp.status,
			headers: resp.headers,
		};
		Ok(result)
	}

	/// 调用ListStyle接口查询某个Bucket下已创建的所有样式
	pub async fn GetStyle(&self, value: &'a str) -> oss::Result<Style> {
		let query = format!("style&styleName={}", value);
		let url = format!("{}?{}", self.options.base_url(), query);
		let resp = self
			.request
			.task()
			.url(&url)
			.method(oss::Method::GET)
			.resourse(&query)
			.send()
			.await?;
		let content = String::from_utf8_lossy(&resp.data);
		let style: Style = quick_xml::de::from_str(&content).unwrap();
		let result = oss::Data {
			data: style,
			status: resp.status,
			headers: resp.headers,
		};
		Ok(result)
	}

	/// 调用DeleteStyle删除某个Bucket下指定的图片样式
	pub async fn DeleteStyle(&self, value: &'a str) -> oss::Result<()> {
		let query = format!("style&styleName={}", value);
		let url = format!("{}?{}", self.options.base_url(), query);
		let resp = self
			.request
			.task()
			.url(&url)
			.method(oss::Method::DELETE)
			.resourse(&query)
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
