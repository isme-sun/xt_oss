use crate::oss;

use self::builders::{DeleteBucketCorsBuilder, GetBucketCorsBuilder, OptionsBuilder, PutBucketCorsBuilder};

pub mod builders {
    use reqwest::header::ORIGIN;

    use crate::{oss::{
        self,
        api::{self, insert_custom_header, insert_header, ApiResponseFrom},
        entities::cors::CORSConfiguration,
        http,
    }, util::AllowedHeaderItem};

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

        pub fn with_config(mut self, value: CORSConfiguration) -> Self {
            self.config = value;
            self
        }

        pub async fn execute(&self) -> api::ApiResult {
            let res = format!("/{}/?{}", self.client.bucket(), "cors");
            let url = format!("{}/?{}", self.client.base_url(), "cors");
            let content = quick_xml::se::to_string(&self.config).unwrap();
            let data = oss::Bytes::from(content);
            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_method(http::Method::PUT)
                .with_resource(&res)
                .with_body(data)
                .execute_timeout(self.client.options.timeout)
                .await?;

            Ok(ApiResponseFrom(resp).to_empty().await)
        }
    }

    pub struct GetBucketCorsBuilder<'a> {
        client: &'a oss::Client<'a>,
    }

    impl<'a> GetBucketCorsBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client) -> Self {
            Self { client }
        }

        pub async fn execute(&self) -> api::ApiResult<CORSConfiguration> {
            let res = format!("/{}/?{}", self.client.bucket(), "cors");
            let url = format!("{}/?{}", self.client.base_url(), "cors");

            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_resource(&res)
                .execute_timeout(self.client.options.timeout)
                .await?;

            Ok(ApiResponseFrom(resp).to_type().await)
        }
    }

    pub struct DeleteBucketCorsBuilder<'a> {
        client: &'a oss::Client<'a>,
    }

    impl<'a> DeleteBucketCorsBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client) -> Self {
            Self { client }
        }

        pub async fn execute(&self) -> api::ApiResult {
            let url = format!("{}/?{}", self.client.base_url(), "cors");
            let res = format!("/{}/?{}", self.client.bucket(), "cors");

            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_resource(&res)
                .with_method(http::Method::DELETE)
                .execute_timeout(self.client.options.timeout)
                .await?;

            Ok(ApiResponseFrom(resp).to_empty().await)
        }
    }

    pub struct OptionsBuilder<'a> {
        client: &'a oss::Client<'a>,
        object: &'a str,
        origin: &'a str,
        request_method: http::Method,
        request_headers: AllowedHeaderItem,
    }

    impl<'a> OptionsBuilder<'a>
    {
        pub(crate) fn new(client: &'a oss::Client, object: &'a str) -> Self {
            Self {
                client,
                object,
                origin: "",
                request_method: http::Method::GET,
                request_headers: AllowedHeaderItem::Any,
            }
        }

        pub fn with_origin(mut self, value:&'a str) -> Self {
            self.origin = value;
            self
        }
        
        pub fn with_request_method(mut self, value:http::Method) -> Self {
            self.request_method = value;
            self
        }

        pub fn with_request_headers(mut self, value:AllowedHeaderItem) -> Self {
            self.request_headers = value;
            self
        }

        pub async fn execute(&self) -> api::ApiResult {
            let res = format!("/{}/{}", self.client.bucket(), self.object);
            let url = self.client.object_url(self.object);

            let mut headers = http::header::HeaderMap::new();
            insert_header(&mut headers, ORIGIN, self.origin);
            insert_custom_header(
                &mut headers,
                "Access-Control-Request-Method",
                self.request_method.to_string(),
            );
            insert_custom_header(
                &mut headers,
                "Access-Control-Request-Headers",
                self.request_headers.to_string(),
            );

            dbg!(&headers);

            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_method(http::Method::OPTIONS)
                .with_headers(headers)
                .with_resource(&res)
                .execute_timeout(self.client.options.timeout)
                .await?;

            Ok(ApiResponseFrom(resp).to_empty().await)
        }
    }
}

/// # 跨域资源共享（CORS）
#[allow(non_snake_case)]
impl<'a> oss::Client<'a> {
    /// 调用PutBucketCors接口为指定的存储空间`Bucket`设置跨域资源共享CORS
    ///`Cross-Origin Resource Sharing`规则
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/putbucketcors)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_cors_put.rs)
    pub fn PutBucketCors(&self) -> PutBucketCorsBuilder {
        PutBucketCorsBuilder::new(&self)
    }

    /// GetBucketCors接口用于获取指定存储空间`Bucket`当前的跨域资源共享CORS
    /// `Cross-Origin Resource Sharing`规则。
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/getbucketcors)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_cors_get.rs)
    pub fn GetBucketCors(&self) -> GetBucketCorsBuilder {
        GetBucketCorsBuilder::new(&self)
    }

    /// DeleteBucketCors用于关闭指定存储空间`Bucket`对应的跨域资源共享CORS
    /// `Cross-Origin Resource Sharing`功能并清空所有规则
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/deletebucketcors)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_cors_del.rs)
    pub fn DeleteBucketCors(&self) -> DeleteBucketCorsBuilder {
        DeleteBucketCorsBuilder::new(&self)
    }

    /// 浏览器在发送跨域请求之前会发送一个preflight请求`Options`给OSS,并带上特定的
    /// 来源域、HTTP方法和header等信息,以决定是否发送真正的请求。Options请求是由浏览
    /// 器自动根据是否跨域来决定是否发送。
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/options)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_cors_options.rs)
    pub fn Options(&self, object: &'a str) -> OptionsBuilder {
        OptionsBuilder::new(&self, object)
    }
}
