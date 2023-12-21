use crate::oss::{self, entities::{RefererConfiguration, inner}, Client};

use super::builders::PutBucketRefererBuilder;
#[allow(non_snake_case)]
impl<'a> Client<'a> {
    /// GetBucketReferer接口用于查看存储空间（Bucket）的防盗链（Referer）相关配置。
    pub async fn GetBucketReferer(&self) -> oss::Result<RefererConfiguration> {
        let res = "referer";
        let url = {
            let base_url = self.options.base_url();
            format!("{base_url}?{res}")
        };

        let resp = self
            .request
            .task()
            .url(&url)
            .resourse(res)
            .send()
            .await
            .unwrap();

        let content = String::from_utf8_lossy(&resp.data);

        let config_inner: inner::RefererConfiguration =
            quick_xml::de::from_str(&content).unwrap();

        let config = RefererConfiguration::from_inner(config_inner);

        let result = oss::Data {
            status: resp.status,
            headers: resp.headers,
            data: config,
        };
        Ok(result)
    }

    /// 调用PutBucketReferer接口设置存储空间（Bucket）级别的Referer访问白名单以及黑名单
    pub fn PutBucketReferer(&self) -> PutBucketRefererBuilder {
        PutBucketRefererBuilder::new(&self)
    }
}
