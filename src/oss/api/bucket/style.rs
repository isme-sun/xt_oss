use crate::oss;

use self::builders::{DeleteStyleBuilder, GetStyleBuilder, ListStyleBuilder, PutStyleBuilder};

pub mod builders {
    use crate::oss::{
        self,
        api::{self, ApiResponseFrom},
        entities::style::{Style, StyleList},
        http,
    };

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

        pub async fn execute(&self) -> api::ApiResult {
            let res = format!("/{}/?{}", self.client.options.bucket, "style");
            let query = format!("style&styleName={}", self.style.name);
            let url = { format!("{}?{}", self.client.options.base_url(), query) };

            let data = oss::Bytes::from(self.style());

            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_method(http::Method::PUT)
                .with_resource(&res)
                .with_body(data)
                .execute()
                .await?;

            Ok(ApiResponseFrom(resp).as_empty().await)
        }
    }

    pub struct ListStyleBuilder<'a> {
        client: &'a oss::Client<'a>,
    }

    impl<'a> ListStyleBuilder<'a> {
        pub fn new(client: &'a oss::Client<'a>) -> Self {
            Self { client }
        }

        pub async fn execute(&self) -> api::ApiResult<StyleList> {
            let res = format!("/{}/?{}", self.client.options.bucket, "style");
            let url = format!("{}/?{}", self.client.options.base_url(), "style");
            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_resource(&res)
                .execute()
                .await?;
            Ok(ApiResponseFrom(resp).as_type().await)
        }
    }

    pub struct GetStyleBuilder<'a> {
        client: &'a oss::Client<'a>,
        name: &'a str,
    }

    impl<'a> GetStyleBuilder<'a> {
        pub fn new(client: &'a oss::Client<'a>, name: &'a str) -> Self {
            Self { client, name }
        }

        pub async fn execute(&self) -> api::ApiResult<Style> {
            let res = format!("/{}/?{}", self.client.options.bucket, "style");
            let url = format!(
                "{}/?{}&styleName={}",
                self.client.options.base_url(),
                "style",
                self.name
            );

            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_method(http::Method::GET)
                .with_resource(&res)
                .execute()
                .await?;
            Ok(ApiResponseFrom(resp).as_type().await)
        }
    }

    pub struct DeleteStyleBuilder<'a> {
        client: &'a oss::Client<'a>,
        name: &'a str,
    }

    impl<'a> DeleteStyleBuilder<'a> {
        pub fn new(client: &'a oss::Client<'a>, name: &'a str) -> Self {
            Self { client, name }
        }

        pub async fn execute(&self) -> api::ApiResult {
            let res = format!("/{}/?{}", self.client.options.bucket, "style");
            let url = format!(
                "{}/?{}&styleName={}",
                self.client.options.base_url(),
                "style",
                self.name
            );

            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_method(http::Method::DELETE)
                .with_resource(&res)
                .execute()
                .await?;
            Ok(ApiResponseFrom(resp).as_empty().await)
        }
    }
}

#[allow(non_snake_case)]
impl<'a> oss::Client<'a> {
    /// 调用PutStyle接口新增图片样式。一个图片样式中可以包含单个或多个图片处理参数
    pub fn PutStyle(&self) -> PutStyleBuilder {
        PutStyleBuilder::new(self)
    }

    /// 调用GetStyle接口查询某个Bucket下指定的样式信息
    pub fn ListStyle(&self) -> ListStyleBuilder {
        ListStyleBuilder::new(self)
    }

    /// 调用ListStyle接口查询某个Bucket下已创建的所有样式
    pub fn GetStyle(&self, name: &'a str) -> GetStyleBuilder {
        GetStyleBuilder::new(self, name)
    }

    /// 调用DeleteStyle删除某个Bucket下指定的图片样式
    pub fn DeleteStyle(&self, name: &'a str) -> DeleteStyleBuilder {
        DeleteStyleBuilder::new(self, name)
    }
}
