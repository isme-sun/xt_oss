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

        pub fn with_name(mut self, value: &'a str) -> Self {
            self.style.name = value.to_string();
            self
        }

        pub fn with_content(mut self, value: &'a str) -> Self {
            self.style.content = value.to_string();
            self
        }

        // pub fn with_category(mut self, value: &'a str) -> Self {
        //     self.style.category = Some(value.to_string());
        //     self
        // }

        fn style(&self) -> String {
            quick_xml::se::to_string(&self.style).unwrap()
        }

        pub async fn execute(&self) -> api::ApiResult {
            let res = format!("/{}/?{}&styleName={}", self.client.bucket(), "style", self.style.name);
            let url = format!("{}?{}&styleName={}", self.client.base_url(), "style", self.style.name);

            let data = oss::Bytes::from(self.style());
            // dbg!(&data);

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

            Ok(ApiResponseFrom(resp).to_empty().await)
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
            let res = format!("/{}/?{}", self.client.bucket(), "style");
            let url = format!("{}/?{}", self.client.base_url(), "style");
            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_resource(&res)
                .execute()
                .await?;
            Ok(ApiResponseFrom(resp).to_type().await)
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
            let res = format!("/{}/?{}&styleName={}", self.client.bucket(), "style", self.name);
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
            Ok(ApiResponseFrom(resp).to_type().await)
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
            let res = format!("/{}/?{}&styleName={}", self.client.bucket(), "style", self.name);
            let url = format!("{}/?{}&styleName={}", self.client.base_url(), "style", self.name);

            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_method(http::Method::DELETE)
                .with_resource(&res)
                .execute()
                .await?;
            Ok(ApiResponseFrom(resp).to_empty().await)
        }
    }
}

/// # 图片样式（Style）
#[allow(non_snake_case)]
impl<'a> oss::Client<'a> {
    /// 调用PutStyle接口新增图片样式。一个图片样式中可以包含单个或多个图片处理参数
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/putstyle)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_style_put.rs)
    pub fn PutStyle(&self) -> PutStyleBuilder {
        PutStyleBuilder::new(self)
    }

    /// 调用ListStyle接口查询某个Bucket下已创建的所有样式
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/deletestyle)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_style_get.rs)
    pub fn GetStyle(&self, name: &'a str) -> GetStyleBuilder {
        GetStyleBuilder::new(self, name)
    }

    /// 调用GetStyle接口查询某个Bucket下指定的样式信息
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/getstyle)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_style_list.rs)
    pub fn ListStyle(&self) -> ListStyleBuilder {
        ListStyleBuilder::new(self)
    }

    /// 调用DeleteStyle删除某个Bucket下指定的图片样式
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/deletestyle)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_style_del.rs)
    pub fn DeleteStyle(&self, name: &'a str) -> DeleteStyleBuilder {
        DeleteStyleBuilder::new(self, name)
    }
}
