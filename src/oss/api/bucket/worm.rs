use crate::oss::arguments::WormConfiguration;
#[allow(unused)]
use crate::oss::{
    self,
    entities::{BucketInfo, BucketStat, ListBucketResult},
    Client, Data, Method, Result,
};

use super::builders::InitiateBucketWormBuilder;

#[allow(non_snake_case)]
impl<'a> Client<'a> {
    /// 调用InitiateBucketWorm接口新建一条合规保留策略。
    #[allow(non_snake_case)]
    pub fn InitiateBucketWorm(&self) -> InitiateBucketWormBuilder {
        InitiateBucketWormBuilder::new(&self)
    }

    /// AbortBucketWorm用于删除未锁定的合规保留策略。
    #[allow(non_snake_case)]
    pub async fn AbortBucketWorm(&self) -> oss::Result<()> {
        let res = "worm";
        let url = format!("{}?{}", self.options.base_url(), res);

        let resp = self
            .request
            .task()
            .method(oss::Method::DELETE)
            .url(&url)
            .resourse(&res)
            .send()
            .await?;

        let result = oss::Data {
            status: resp.status,
            headers: resp.headers,
            data: (),
        };
        Ok(result)
    }

    /// CompleteBucketWorm用于锁定合规保留策略。
    #[allow(non_snake_case)]
    pub async fn CompleteBucketWorm(&self, worm_id: &'a str) -> oss::Result<()> {
        let res = format!("wormId={}", worm_id);
        let url = format!("{}/?{}", self.options.base_url(), res);

        let resp = self
            .request
            .task()
            .method(oss::Method::POST)
            .url(&url)
            .resourse(&res)
            .send()
            .await?;

        let result = oss::Data {
            status: resp.status,
            headers: resp.headers,
            data: (),
        };
        Ok(result)
    }

    /// ExtendBucketWorm用于延长已锁定的合规保留策略对应Bucket中Object的保留天数。
    #[allow(non_snake_case)]
    pub fn ExtendBucketWorm() {
        todo!()
    }

    /// GetBucketWorm用于获取指定存储空间（Bucket）的合规保留策略信息。
    #[allow(non_snake_case)]
    pub async fn GetBucketWorm(&self) -> oss::Result<WormConfiguration> {
        let res = "worm";
        let url = format!("{}?{}", self.options.base_url(), res);

        let resp = self
            .request
            .task()
            .method(oss::Method::GET)
            .url(&url)
            .resourse(&res)
            .send()
            .await?;

        let content = String::from_utf8_lossy(&resp.data);
        let config: WormConfiguration = serde_xml_rs::from_str(&content).unwrap();

        let result = oss::Data {
            status: resp.status,
            headers: resp.headers,
            data: config,
        };
        Ok(result)
    }
}
