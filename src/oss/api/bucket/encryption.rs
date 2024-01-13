use crate::oss::entities::encryption::ServerSideEncryptionRule;
#[allow(unused)]
use crate::oss::{self, Client, Data, Method, Result};

use super::builders::PutBucketEncryptionBuilder;

#[allow(non_snake_case)]
impl<'a> Client<'a> {
    /// PutBucketEncryption接口用于配置存储空间（Bucket）的加密规则。
    pub fn PutBucketEncryption(&self) -> PutBucketEncryptionBuilder {
        PutBucketEncryptionBuilder::new(&self)
    }

    /// GetBucketEncryption接口用于获取存储空间（Bucket）的加密规则。
    pub async fn GetBucketEncryption(&self) -> oss::Result<ServerSideEncryptionRule> {
        let res = "encryption";
        let url = format!("{}/?{}", self.options.base_url(), res);
        let resp = self
            .request
            .task()
            .url(&url)
            .resourse(res)
            .send()
            .await?;

        let content = String::from_utf8_lossy(&resp.data);
        let rule: ServerSideEncryptionRule= quick_xml::de::from_str(&content).unwrap();
        let result = oss::Data {
            status: resp.status,
            headers: resp.headers,
            data: rule
        };
        Ok(result)
    }

    /// DeleteBucketEncryption接口用于删除Bucket加密规则。
    pub async fn DeleteBucketEncryption(&self) -> oss::Result<()> {
        let res = "encryption";
        let url = format!("{}/?{}", self.options.base_url(), res);
        let resp = self
            .request
            .task()
            .url(&url)
            .method(oss::Method::DELETE)
            .resourse(res)
            .send()
            .await?;

        let result = oss::Data {
            status: resp.status,
            headers: resp.headers,
            data: ()
        };
        Ok(result)
    }
}
