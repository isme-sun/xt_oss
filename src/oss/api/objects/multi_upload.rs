// use crate::OssClient;
use crate::oss::{api::objects::multi_upload::builders::AbortMultipartUploadBuilder, Client};
use builders::{
    CompleteMultipartUploadBuilder, InitiateMultipartUploadBuilder, ListMultipartUploadsBuilder, UploadPartBuilder,
    UploadPartCopyBuilder,
};

use self::builders::ListPartsBuilder;

#[allow(unused)]
pub mod builders {
    use std::collections::HashMap;

    use chrono::{DateTime, Utc};

    use crate::oss::{
        self,
        api::{self, insert_custom_header, insert_header, ApiResponseFrom},
        entities::{multi_upload::InitiateMultipartUploadResult, ServerSideEncryption, StorageClass},
        http::{
            self,
            header::{CACHE_CONTROL, CONTENT_DISPOSITION, CONTENT_ENCODING, CONTENT_TYPE, EXPECT},
        },
    };

    #[derive(Debug, Default)]
    struct InitiateMultipartUploadBuilderHeaders {
        cache_control: Option<http::CacheControl>,
        content_disposition: Option<http::ContentDisposition>,
        content_encoding: Option<http::ContentEncoding>,
        expires: Option<DateTime<Utc>>,
        content_type: Option<String>,
        forbid_overwrite: Option<bool>,
        encryption: Option<ServerSideEncryption>,
        data_encryption: Option<String>,
        encryption_key_id: Option<String>,
        storage_class: Option<StorageClass>,
        oss_tagging: HashMap<String, String>,
    }

    pub struct InitiateMultipartUploadBuilder<'a> {
        client: &'a oss::Client<'a>,
        object: &'a str,
        encoding_type: Option<&'a str>,
        headers: InitiateMultipartUploadBuilderHeaders,
    }

    impl<'a> InitiateMultipartUploadBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client, object: &'a str) -> Self {
            Self {
                client,
                object,
                encoding_type: None,
                headers: InitiateMultipartUploadBuilderHeaders::default(),
            }
        }

        pub fn with_content_type(mut self, value: &'a str) -> Self {
            self.headers.content_type = Some(value.to_string());
            self
        }

        pub fn with_cache_control(mut self, value: http::CacheControl) -> Self {
            self.headers.cache_control = Some(value);
            self
        }

        pub fn with_content_disposition(mut self, value: http::ContentDisposition) -> Self {
            self.headers.content_disposition = Some(value);
            self
        }

        pub fn with_content_encoding(mut self, value: http::ContentEncoding) -> Self {
            self.headers.content_encoding = Some(value);
            self
        }

        pub fn with_expires(mut self, value: DateTime<Utc>) -> Self {
            self.headers.expires = Some(value);
            self
        }

        pub fn with_forbid_overwrite(mut self, value: bool) -> Self {
            self.headers.forbid_overwrite = Some(value);
            self
        }

        pub fn with_encryption(mut self, value: ServerSideEncryption) -> Self {
            self.headers.encryption = Some(value);
            self
        }

        pub fn with_data_encryption(mut self, value: &'a str) -> Self {
            self.headers.data_encryption = Some(value.to_string());
            self
        }

        pub fn with_encryption_key_id(mut self, value: &'a str) -> Self {
            self.headers.encryption_key_id = Some(value.to_string());
            self
        }

        pub fn with_storage_class(mut self, value: StorageClass) -> Self {
            self.headers.storage_class = Some(value);
            self
        }

        pub fn with_oss_tagging(mut self, key: &'a str, value: &'a str) -> Self {
            self.headers.oss_tagging.insert(key.to_string(), value.to_string());
            self
        }

        fn headers(&self) -> http::HeaderMap {
            let mut headers = http::HeaderMap::new();

            if let Some(cache_control) = &self.headers.cache_control {
                insert_header(&mut headers, CACHE_CONTROL, cache_control);
            }

            if let Some(content_disposition) = &self.headers.content_disposition {
                insert_header(&mut headers, CONTENT_DISPOSITION, content_disposition);
            }

            if let Some(content_type) = &self.headers.cache_control {
                insert_header(&mut headers, CONTENT_TYPE, content_type);
            }

            if let Some(content_encoding) = &self.headers.content_encoding {
                insert_header(&mut headers, CONTENT_ENCODING, content_encoding);
            }

            if let Some(expires) = &self.headers.expires {
                insert_header(&mut headers, EXPECT, expires.format(oss::GMT_DATE_FMT));
            }

            if let Some(content_type) = &self.headers.content_type {
                insert_header(&mut headers, CONTENT_TYPE, content_type);
            }

            if let Some(forbid_overwrite) = &self.headers.forbid_overwrite {
                insert_custom_header(&mut headers, "x-oss-forbid-overwrite", forbid_overwrite);
            }

            if let Some(encryption) = &self.headers.encryption {
                insert_custom_header(&mut headers, "x-oss-server-side-encryption", encryption);
            }

            if let Some(data_encryption) = &self.headers.data_encryption {
                headers.insert("x-oss-server-side-data-encryption", data_encryption.parse().unwrap());
            }

            if let Some(encryption_key_id) = &self.headers.encryption_key_id {
                insert_custom_header(&mut headers, "x-oss-server-side-encryption-key-id", encryption_key_id);
            }

            if let Some(storage_class) = &self.headers.storage_class {
                insert_custom_header(&mut headers, "x-oss-storage-class", storage_class);
            }

            if !self.headers.oss_tagging.is_empty() {
                let value = serde_qs::to_string(&self.headers.oss_tagging).expect("Failed to serialize tags");
                insert_custom_header(&mut headers, "x-oss-tagging", value);
            }

            headers
        }

        pub async fn execute(&self) -> api::ApiResult<InitiateMultipartUploadResult> {
            let mut res = format!("/{}/{}?{}", self.client.bucket(), self.object, "uploads");
            let mut url = format!("{}?{}", self.client.object_url(self.object), "uploads");
            if let Some(encoding_type) = self.encoding_type {
                res = format!("{}&encoding_type={}", res, encoding_type);
                url = format!("{}&encoding_type={}", res, encoding_type);
            }

            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_method(http::Method::POST)
                .with_headers(self.headers())
                .with_resource(&res)
                .execute_timeout(self.client.timeout())
                .await?;

            Ok(ApiResponseFrom(resp).to_type().await)
        }
    }

    pub struct UploadPartBuilder<'a> {
        client: &'a oss::Client<'a>,
    }

    impl<'a> UploadPartBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client) -> Self {
            Self { client }
        }

        pub async fn execute(&self) -> api::ApiResult {
            todo!()
        }
    }

    pub struct UploadPartCopyBuilder<'a> {
        client: &'a oss::Client<'a>,
    }

    impl<'a> UploadPartCopyBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client) -> Self {
            Self { client }
        }

        pub async fn execute(&self) -> api::ApiResult {
            todo!()
        }
    }

    pub struct CompleteMultipartUploadBuilder<'a> {
        client: &'a oss::Client<'a>,
    }

    impl<'a> CompleteMultipartUploadBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client) -> Self {
            Self { client }
        }

        pub async fn execute(&self) -> api::ApiResult {
            todo!()
        }
    }

    pub struct AbortMultipartUploadBuilder<'a> {
        client: &'a oss::Client<'a>,
    }

    impl<'a> AbortMultipartUploadBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client) -> Self {
            Self { client }
        }

        pub async fn execute(&self) -> api::ApiResult {
            todo!()
        }
    }

    pub struct ListMultipartUploadsBuilder<'a> {
        client: &'a oss::Client<'a>,
    }

    impl<'a> ListMultipartUploadsBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client) -> Self {
            Self { client }
        }

        pub async fn execute(&self) -> api::ApiResult {
            todo!()
        }
    }

    pub struct ListPartsBuilder<'a> {
        client: &'a oss::Client<'a>,
    }

    impl<'a> ListPartsBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client) -> Self {
            Self { client }
        }

        pub async fn execute(&self) -> api::ApiResult {
            todo!()
        }
    }
}

/// 基础操作
#[allow(non_snake_case)]
impl<'a> Client<'a> {
    /// 使用Multipart Upload模式传输数据前，您必须先调用InitiateMultipartUpload接口来通知OSS初始化一
    /// 个Multipart Upload事件。
    ///
    /// - [official docs]()
    /// - [xtoss example]()
    pub fn InitiateMultipartUpload(&self, object: &'a str) -> InitiateMultipartUploadBuilder {
        InitiateMultipartUploadBuilder::new(self, object)
    }

    /// 初始化一个MultipartUpload后，调用UploadPart接口根据指定的Object名和uploadId来分块（Part）上传数据。
    ///
    /// - [official docs]()
    /// - [xtoss example]()
    pub fn UploadPart(&self) -> UploadPartBuilder {
        UploadPartBuilder::new(self)
    }

    /// 通过在UploadPart请求的基础上增加一个请求头x-oss-copy-source来调用UploadPartCopy接口，实现从一个
    /// 已存在的Object中拷贝数据来上传一个Part。
    ///
    /// - [official docs]()
    /// - [xtoss example]()
    pub fn UploadPartCopy(&self) -> UploadPartCopyBuilder {
        UploadPartCopyBuilder::new(self)
    }

    /// 在将所有数据Part都上传完成后，您必须调用CompleteMultipartUpload接口来完成整个文件的分片上传。
    ///
    /// - [official docs]()
    /// - [xtoss example]()
    pub fn CompleteMultipartUpload(&self) -> CompleteMultipartUploadBuilder {
        CompleteMultipartUploadBuilder::new(self)
    }

    /// AbortMultipartUpload接口用于取消MultipartUpload事件并删除对应的Part数据。
    ///
    /// - [official docs]()
    /// - [xtoss example]()
    pub fn AbortMultipartUpload(&self) -> AbortMultipartUploadBuilder {
        AbortMultipartUploadBuilder::new(self)
    }

    /// 调用ListMultipartUploads接口列举所有执行中的Multipart Upload事件，即已经初始化但还未完成
    ///（Complete）或者还未中止（Abort）的Multipart Upload事件。
    ///
    /// - [official docs]()
    /// - [xtoss example]()
    pub fn ListMultipartUploads(&self) -> ListMultipartUploadsBuilder {
        ListMultipartUploadsBuilder::new(self)
    }

    /// ListParts接口用于列举指定Upload ID所属的所有已经上传成功Part。
    ///
    /// - [official docs]()
    /// - [xtoss example]()
    pub fn ListParts(&self) -> ListPartsBuilder {
        ListPartsBuilder::new(self)
    }
}
