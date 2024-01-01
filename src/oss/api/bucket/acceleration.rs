use crate::oss::entities::TransferAccelerationConfiguration;
#[allow(unused)]
use crate::oss::{
    self,
    entities::{BucketInfo, BucketStat},
    Client, Data, Method, Result,
};

#[allow(non_snake_case)]
impl<'a> Client<'a> {
    /// 接口用于为存储空间（Bucket）配置传输加速。开启传输加速后，可提升全球各地用户对OSS的访问速度，适用于远距离数据传输、GB或TB级大文件上传和下载的场景。
    pub async fn PutBucketTransferAcceleration(&self, value: bool) -> oss::Result<()> {
        let res = "transferAcceleration";
        let config = TransferAccelerationConfiguration { enabled: value };
        let url = format!("{}/?{}", self.options.base_url(), res);
        let data = oss::Bytes::from(quick_xml::se::to_string(&config).unwrap());

        let resp = self
            .request
            .task()
            .url(&url)
            .resourse(res)
            .method(oss::Method::PUT)
            .body(data)
            .send()
            .await?;

        let result = oss::Data {
            status: resp.status,
            headers: resp.headers,
            data: (),
        };
        Ok(result)
    }

    /// 接口用于获取目标存储空间（Bucket）的传输加速配置。
    pub async fn GetBucketTransferAcceleration(
        &self,
    ) -> oss::Result<TransferAccelerationConfiguration> {
        let res = "transferAcceleration";
        let url = format!("{}/?{}", self.options.base_url(), res);
        let resp = self.request.task().url(&url).resourse(res).send().await?;

        let content = String::from_utf8_lossy(&resp.data);
        let data: TransferAccelerationConfiguration = quick_xml::de::from_str(&content).unwrap();

        let result = oss::Data {
            status: resp.status,
            headers: resp.headers,
            data,
        };
        Ok(result)
    }
}
