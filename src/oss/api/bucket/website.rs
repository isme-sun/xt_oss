use crate::oss;

use self::builders::{
    DeleteBucketWebsiteBuilder, GetBucketWebsiteBuilder, PutBucketWebsiteBuilder,
};

pub mod builders {
    use crate::oss::{
        self,
        api::{self, ApiResponseFrom},
        entities::website::{
            ErrorDocument, IndexDocument, RoutingRule, RoutingRules, WebsiteConfiguration,
        },
        http,
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

        pub async fn execute(&self) -> api::ApiResult<()> {
            let res = format!("/{}/?{}", self.client.options.bucket, "website");
            let url = format!("{}/?{}", self.client.options.base_url(), "website");

            let config = self.config();
            let data = oss::Bytes::from(config);

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

    pub struct GetBucketWebsiteBuilder<'a> {
        client: &'a oss::Client<'a>,
    }

    impl<'a> GetBucketWebsiteBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client) -> Self {
            Self { client }
        }

        pub async fn execute(&self) -> api::ApiResult<WebsiteConfiguration> {
            let res = format!("/{}/?{}", self.client.options.bucket, "website");
            let url = format!("{}/?{}", self.client.options.base_url(), "website");
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

    pub struct DeleteBucketWebsiteBuilder<'a> {
        client: &'a oss::Client<'a>,
    }

    impl<'a> DeleteBucketWebsiteBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client) -> Self {
            Self { client }
        }

        pub async fn execute(&self) -> api::ApiResult<WebsiteConfiguration> {
            let res = format!("/{}/?{}", self.client.options.bucket, "website");
            let url = format!("{}/?{}", self.client.options.base_url(), "website");
            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_method(http::Method::DELETE)
                .with_resource(&res)
                .execute()
                .await?;
            Ok(ApiResponseFrom(resp).as_type().await)
        }
    }
}

/// ## 静态网站（Website）
#[allow(non_snake_case)]
impl<'a> oss::Client<'a> {
    /// 调用PutBucketWebsite接口将存储空间（Bucket）设置为静态网站托管模式并设置跳转
    /// 规则（RoutingRule
    ///
    /// **Example**:
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
        PutBucketWebsiteBuilder::new(self)
    }

    /// 调用GetBucketWebsite接口查看存储空间（Bucket）的静态网站托管状态以及跳转规则
    pub fn GetBucketWebsite(&self) -> GetBucketWebsiteBuilder {
        GetBucketWebsiteBuilder::new(self)
    }

    /// DeleteBucketWebsite接口用于关闭存储空间（Bucket）的静态网站托管模式以及跳转规则。只有Bucket的拥有者才能关闭Bucket的静态网站托管模式。
    pub fn DeleteBucketWebsite(&self) -> DeleteBucketWebsiteBuilder {
        DeleteBucketWebsiteBuilder::new(self)
    }
}

#[cfg(test)]
pub mod tests {
    use crate::oss;

    use super::builders::PutBucketWebsiteBuilder;

    #[test]
    fn test_put_bucket_website_builder() {
        let client = oss::Client::new(oss::Options::default());
        let builder = PutBucketWebsiteBuilder::new(&client).with_default();
        let left = r#"<WebsiteConfiguration><IndexDocument><Suffix>index.html</Suffix><SupportSubDir>true</SupportSubDir><Type>0</Type></IndexDocument><ErrorDocument><Key>error.html</Key><HttpStatus>404</HttpStatus></ErrorDocument></WebsiteConfiguration>"#;
        let right = builder.config();
        assert_eq!(left, right);
    }
}
