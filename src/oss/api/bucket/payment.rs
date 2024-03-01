pub mod builders {
    use oss::http;

    use crate::oss::{
        self,
        api::{self, ApiResponseFrom},
        entities::payment::{Payer, RequestPaymentConfiguration},
    };
    pub struct PutBucketRequestPaymentBuilder<'a> {
        client: &'a oss::Client<'a>,
        config: RequestPaymentConfiguration,
    }

    impl<'a> PutBucketRequestPaymentBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client, payer: Payer) -> Self {
            Self {
                client,
                config: RequestPaymentConfiguration { payer },
            }
        }

        fn config(&self) -> String {
            quick_xml::se::to_string(&self.config).unwrap()
        }

        pub async fn execute(&self) -> api::ApiResult {
            let res = format!("/{}/?requestPayment", self.client.bucket());
            let url = format!("{}?requestPayment", self.client.base_url());
            let content = oss::Bytes::from(self.config());
            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_method(http::Method::PUT)
                .with_body(content)
                .with_resource(&res)
                .execute_timeout(self.client.timeout())
                .await?;

            Ok(ApiResponseFrom(resp).to_empty().await)
        }
    }

    pub struct GetBucketRequestPaymentBuilder<'a> {
        client: &'a oss::Client<'a>,
    }

    impl<'a> GetBucketRequestPaymentBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client) -> Self {
            Self { client }
        }

        pub async fn execute(&self) -> api::ApiResult<RequestPaymentConfiguration> {
            let res = format!("/{}/?requestPayment", self.client.bucket());
            let url = format!("{}/?requestPayment", self.client.base_url());

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
}

use crate::oss::{self, entities::payment::Payer};

use self::builders::{GetBucketRequestPaymentBuilder, PutBucketRequestPaymentBuilder};
/// # 请求者付费`RequestPayment`
#[allow(non_snake_case)]
impl<'a> oss::Client<'a> {
    /// PutBucketRequestPayment接口用于设置请求者付费模式。
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/putbucketrequestpayment)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_payment_put.rs)
    pub fn PutBucketRequestPayment(&self, payer: Payer) -> PutBucketRequestPaymentBuilder {
        PutBucketRequestPaymentBuilder::new(self, payer)
    }

    /// GetBucketRequestPayment接口用于获取请求者付费模式配置信息。
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/getbucketrequestpayment)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_payment_get.rs)
    pub fn GetBucketRequestPayment(&self) -> GetBucketRequestPaymentBuilder {
        GetBucketRequestPaymentBuilder::new(&self)
    }
}
