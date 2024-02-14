use crate::oss;

use self::builders::{GetBucketTransferAccelerationBuilder, PutBucketTransferAccelerationBuilder};

pub mod builders {

    use crate::oss::{
        self,
        api::{self, ApiResponseFrom},
        entities::acceleration::TransferAccelerationConfiguration,
        http,
    };

    pub struct PutBucketTransferAccelerationBuilder<'a> {
        client: &'a oss::Client<'a>,
        enabled: Option<bool>,
    }

    impl<'a> PutBucketTransferAccelerationBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client, value: bool) -> Self {
            Self {
                client,
                enabled: Some(value),
            }
        }

        pub async fn execute(&self) -> api::ApiResult<()> {
            let res = format!("/{}/?transferAcceleration", self.client.bucket());
            let url = format!("{}/?transferAcceleration", self.client.base_url());

            let config = TransferAccelerationConfiguration {
                enabled: self.enabled.unwrap(),
            };
            let data = oss::Bytes::from(quick_xml::se::to_string(&config).unwrap());
            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_resource(&res)
                .with_method(http::Method::PUT)
                .with_body(data)
                .execute_timeout(self.client.timeout())
                .await?;
            Ok(ApiResponseFrom(resp).as_empty().await)
        }
    }

    //----------------------------------------------
    pub struct GetBucketTransferAccelerationBuilder<'a> {
        client: &'a oss::Client<'a>,
    }

    impl<'a> GetBucketTransferAccelerationBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client) -> Self {
            Self {
                client,
            }
        }

        pub async fn execute(&self) -> api::ApiResult<TransferAccelerationConfiguration> {
            let res = format!("/{}/?transferAcceleration", self.client.bucket());
            let url = format!("{}/?transferAcceleration", self.client.base_url());
            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_resource(&res)
                .execute_timeout(self.client.timeout())
                .await?;

            Ok(ApiResponseFrom(resp).as_type().await)
        }
    }
}

/// # 传输加速（TransferAcceleration）
#[allow(non_snake_case)]
impl<'a> oss::Client<'a> {
    /// 接口用于为存储空间（Bucket）配置传输加速。开启传输加速后，可提升全球各地用户对OSS的访问速度，
    /// 适用于远距离数据传输、GB或TB级大文件上传和下载的场景。
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/putbuckettransferacceleration)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_transfer_acceleration_put)
    pub fn PutBucketTransferAcceleration(
        &self,
        value: bool,
    ) -> PutBucketTransferAccelerationBuilder<'_> {
        PutBucketTransferAccelerationBuilder::new(&self, value)
    }

    /// 接口用于获取目标存储空间（Bucket）的传输加速配置
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/getbuckettransferacceleration)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_transfer_acceleration_get)
    pub fn GetBucketTransferAcceleration(&self) -> GetBucketTransferAccelerationBuilder {
        GetBucketTransferAccelerationBuilder::new(&self)
    }
}
