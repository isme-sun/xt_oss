#[allow(unused)]
use crate::oss::{self, api, entities::acceleration::TransferAccelerationConfiguration};

use self::builder::PutBucketTransferAccelerationBuilder;

pub mod builder {

    use crate::oss::{
        self,
        api::{self, into_api_result},
        entities::acceleration::TransferAccelerationConfiguration,
        http,
    };

    pub struct PutBucketTransferAccelerationBuilder<'a> {
        client: &'a oss::Client<'a>,
        enabled: Option<bool>,
        timeout: Option<u64>,
    }

    impl<'a> PutBucketTransferAccelerationBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client, value: bool) -> Self {
            Self {
                client,
                enabled: Some(value),
                timeout: None,
            }
        }

        pub fn with_timeout(mut self, value: u64) -> Self {
            self.timeout = Some(value);
            self
        }

        pub async fn execute(&self) -> api::Result<()> {
            let url = format!("{}/?transferAcceleration", self.client.options.base_url());
            let config = TransferAccelerationConfiguration {
                enabled: self.enabled.unwrap(),
            };
            let data = oss::Bytes::from(quick_xml::se::to_string(&config).unwrap());

            let res = format!("/{}/?transferAcceleration", self.client.options.bucket);

            let task = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_resource(&res)
                .with_method(http::Method::PUT)
                .with_body(data);

            let resp = match self.timeout {
                Some(timeout) => task.execute_timeout(timeout).await,
                None => task.execute().await,
            };

            into_api_result::<()>(resp).await
        }
    }

    //----------------------------------------------
    // pub struct GetBucketTransferAccelerationBuilder<'a> {
    //     client: &'a oss::Client<'a>,
    //     timeout: Option<u64>,
    // }


}

#[allow(non_snake_case)]
impl<'a> oss::Client<'a> {
    /// 接口用于为存储空间（Bucket）配置传输加速。开启传输加速后，可提升全球各地用户对OSS的访问速度，适用于远距离数据传输、GB或TB级大文件上传和下载的场景。
    pub fn PutBucketTransferAcceleration(
        &self,
        value: bool,
    ) -> PutBucketTransferAccelerationBuilder<'_> {
        PutBucketTransferAccelerationBuilder::new(&self, value)
    }

    // 接口用于获取目标存储空间（Bucket）的传输加速配置。
    // pub async fn GetBucketTransferAcceleration(
    //     &self,
    // ) -> oss::Result<TransferAccelerationConfiguration> {
    //     let res = "transferAcceleration";
    //     let url = format!("{}/?{}", self.options.base_url(), res);
    //     let resp = self.request.task().url(&url).resourse(res).send().await?;

    //     let content = String::from_utf8_lossy(&resp.data);
    //     let data = quick_xml::de::from_str(&content).unwrap();

    //     let result = oss::Data {
    //         status: resp.status,
    //         headers: resp.headers,
    //         data,
    //     };
    //     Ok(result)
    // }
}
