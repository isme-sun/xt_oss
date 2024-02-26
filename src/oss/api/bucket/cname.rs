use crate::oss;
use builders::{CreateCnameTokenBuilder, GetCnameTokenBuilder, ListCnameBuilder, PutCnameBuilder};

use self::builders::DeleteCnameBuilder;

pub mod builders {
    use crate::oss::{
        self,
        api::{self, ApiResponseFrom},
        entities::cname::{
            builders::BucketCnameConfigurationBuilder, BucketCnameConfiguration, Cname, CnameToken,
            ListCnameResult,
        },
        http,
    };

    pub struct CreateCnameTokenBuilder<'a> {
        client: &'a oss::Client<'a>,
        cname: &'a str,
    }

    impl<'a> CreateCnameTokenBuilder<'a> {
        pub fn new(client: &'a oss::Client, cname: &'a str) -> Self {
            Self { client, cname }
        }

        fn config(&self) -> String {
            let config = BucketCnameConfiguration {
                cname: Cname {
                    domain: self.cname.to_string(),
                    ..Cname::default()
                },
            };
            quick_xml::se::to_string(&config).unwrap()
        }

        pub async fn execute(&self) -> api::ApiResult<CnameToken> {
            let res = format!("/{}/?cname&comp=token", self.client.bucket());
            let url = format!("{}/?cname&comp=token", &self.client.base_url());

            let data = oss::Bytes::from(self.config());

            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_method(http::Method::POST)
                .with_resource(&res)
                .with_body(data)
                .execute_timeout(self.client.timeout())
                .await?;

            Ok(ApiResponseFrom(resp).to_type().await)
        }
    }

    pub struct GetCnameTokenBuilder<'a> {
        client: &'a oss::Client<'a>,
        cname: &'a str,
    }

    impl<'a> GetCnameTokenBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client, cname: &'a str) -> Self {
            Self { client, cname }
        }

        pub async fn execute(&self) -> api::ApiResult<CnameToken> {
            let res = format!("/{}/?cname={}&comp=token", self.client.bucket(), self.cname);
            let url = format!(
                "{}/?cname={}&comp=token",
                self.client.base_url(),
                self.cname
            );

            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_resource(&res)
                .execute_timeout(self.client.timeout())
                .await?;

            Ok(ApiResponseFrom(resp).to_type().await)
        }
    }

    pub struct PutCnameBuilder<'a> {
        client: &'a oss::Client<'a>,
        bucket_cname_configuration: BucketCnameConfiguration,
    }

    impl<'a> PutCnameBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client) -> Self {
            Self {
                client,
                bucket_cname_configuration: BucketCnameConfiguration::default(),
            }
        }

        pub fn with_config(mut self, value: BucketCnameConfiguration) -> Self {
            self.bucket_cname_configuration = value;
            self
        }

        fn config(&self) -> String {
            quick_xml::se::to_string(&self.bucket_cname_configuration).unwrap()
        }

        pub async fn execute(&self) -> api::ApiResult {
            let res = format!("/{}/?cname&comp=add", self.client.bucket());
            let url = format!("{}/?cname&comp=add", self.client.base_url());

            let data = oss::Bytes::from(self.config());

            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_method(http::Method::POST)
                .with_body(data)
                .with_resource(&res)
                .execute_timeout(self.client.timeout())
                .await?;

            Ok(ApiResponseFrom(resp).to_empty().await)
        }
    }

    pub struct ListCnameBuilder<'a> {
        client: &'a oss::Client<'a>,
    }

    impl<'a> ListCnameBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client) -> Self {
            Self { client }
        }

        pub async fn execute(&self) -> api::ApiResult<ListCnameResult> {
            let res = format!("/{}/?{}", self.client.bucket(), "cname");
            let url = format!("{}/?{}", self.client.base_url(), "cname");

            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_resource(&res)
                .execute_timeout(self.client.timeout())
                .await?;

            Ok(ApiResponseFrom(resp).to_type().await)
        }
    }

    pub struct DeleteCnameBuilder<'a> {
        client: &'a oss::Client<'a>,
        cname: &'a str,
    }

    impl<'a> DeleteCnameBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client, cname: &'a str) -> Self {
            Self { client, cname }
        }

        fn config(&self) -> String {
            let config = BucketCnameConfigurationBuilder::new()
                .with_domain(self.cname)
                .build();
            quick_xml::se::to_string(&config).unwrap()
        }

        pub async fn execute(&self) -> api::ApiResult {
            let res = format!("/{}/?cname&comp=delete", self.client.bucket());
            let url = format!("{}/?cname&comp=delete", self.client.base_url());

            let data = oss::Bytes::from(self.config());

            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_resource(&res)
                .with_method(http::Method::POST)
                .with_body(data)
                .execute_timeout(self.client.timeout())
                .await?;

            Ok(ApiResponseFrom(resp).to_empty().await)
        }
    }
}

/// # 自定义域名`CNAME``
#[allow(non_snake_case)]
impl<'a> oss::Client<'a> {
    /// 调用CreateCnameToken接口创建域名所有权验证所需的CnameToken
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/createcnametoken)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_cname_create_token.rs)
    pub fn CreateCnameToken(&self, cname: &'a str) -> CreateCnameTokenBuilder<'_> {
        CreateCnameTokenBuilder::new(self, cname)
    }

    /// 调用GetCnameToken接口获取已创建的CnameToken
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/getcnametoken)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_cname_get_token.rs)
    pub fn GetCnameToken(&self, cname: &'a str) -> GetCnameTokenBuilder<'_> {
        GetCnameTokenBuilder::new(self, cname)
    }

    /// 调用PutCname接口为某个存储空间(Bucket)绑定自定义域名
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/putcname)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_cname_put.rs)
    pub fn PutCname(&self) -> PutCnameBuilder<'_> {
        PutCnameBuilder::new(self)
    }

    /// 调用ListCname接口用于查询某个存储空间(Bucket)下绑定的所有的自定义域名(Cname)列表
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/listcname)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_cname_list.rs)
    pub fn ListCname(&self) -> ListCnameBuilder<'_> {
        ListCnameBuilder::new(self)
    }

    /// 调用DeleteCname接口删除某个存储空间(Bucket)已绑定的Cname
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/deletecname)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_cname_del.rs)
    pub fn DeleteCname(&self, cname: &'a str) -> DeleteCnameBuilder<'_> {
        DeleteCnameBuilder::new(self, cname)
    }
}
