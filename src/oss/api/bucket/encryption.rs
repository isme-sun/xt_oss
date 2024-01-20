use crate::oss::{self, entities::encryption::ServerSideEncryptionRule, Client};

use self::builder::PutBucketEncryptionBuilder;
pub mod builder {
    use crate::oss::{
        self,
        entities::encryption::{
            ApplyServerSideEncryptionByDefault, SSEAlgorithm, ServerSideEncryptionRule,
        },
    };

    #[allow(unused)]
    pub struct PutBucketEncryptionBuilder<'a> {
        client: &'a oss::Client<'a>,
        algorithm: SSEAlgorithm,
        data_encryption: Option<&'a str>,
        master_key_id: Option<&'a str>,
    }

    #[allow(unused)]
    impl<'a> PutBucketEncryptionBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client) -> Self {
            Self {
                client,
                algorithm: SSEAlgorithm::default(),
                data_encryption: None,
                master_key_id: None,
            }
        }

        pub fn algorithm(mut self, value: SSEAlgorithm) -> Self {
            self.algorithm = value;
            self
        }

        pub fn data_encryption(mut self, value: &'a str) -> Self {
            self.data_encryption = Some(value);
            self
        }

        pub fn master_key_id(mut self, value: &'a str) -> Self {
            self.master_key_id = Some(value);
            self
        }

        pub async fn send(&self) -> oss::Result<()> {
            let res = "encryption";
            let url = format!("{}/?{}", self.client.options.base_url(), res);

            let mut content = ServerSideEncryptionRule {
                apply_server_side_encryption_by_default: ApplyServerSideEncryptionByDefault {
                    sse_algorithm: self.algorithm,
                    kms_data_encryption: if let Some(enc) = self.data_encryption {
                        Some(enc.to_string())
                    } else {
                        None
                    },
                    kms_master_key_id: if let Some(key_id) = self.master_key_id {
                        Some(key_id.to_string())
                    } else {
                        None
                    },
                },
            };

            let data = oss::Bytes::from(quick_xml::se::to_string(&content).unwrap());

            let resp = self
                .client
                .request
                .task()
                .url(&url)
                .method(oss::Method::PUT)
                .resourse(res)
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
    }
}

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
        let resp = self.request.task().url(&url).resourse(res).send().await?;

        let content = String::from_utf8_lossy(&resp.data);
        let rule: ServerSideEncryptionRule = quick_xml::de::from_str(&content).unwrap();
        let result = oss::Data {
            status: resp.status,
            headers: resp.headers,
            data: rule,
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
            data: (),
        };
        Ok(result)
    }
}
