// use crate::OssClient;
use crate::oss::{api::objects::multi_upload::builders::AbortMultipartUploadBuilder, Client};
use builders::{
    CompleteMultipartUploadBuilder, InitiateMultipartUploadBuilder, ListMultipartUploadsBuilder,
    UploadPartBuilder, UploadPartCopyBuilder,
};

use self::builders::ListPartsBuilder;

#[allow(unused)]
pub mod builders {
    use std::collections::HashMap;

    use chrono::{DateTime, Utc};
    use reqwest::header::CONTENT_LENGTH;
    use serde::{Deserialize, Serialize};

    use crate::oss::{
        self,
        api::{
            self, bucket::stand::builders::ListObjectQuery, insert_custom_header, insert_header,
            ApiResponseFrom,
        },
        entities::{
            multi_upload::{
                CompleteMultipartUploadResult, InitiateMultipartUploadResult,
                ListMultipartUploadsResult, ListPartsResult,
            },
            object, ServerSideEncryption, StorageClass,
        },
        http::{
            self,
            header::{CACHE_CONTROL, CONTENT_DISPOSITION, CONTENT_ENCODING, CONTENT_TYPE, EXPECT},
        },
    };

    #[derive(Debug, Default)]
    struct InitiateMultipartUploadBuilderHeaders<'a> {
        cache_control: Option<http::CacheControl>,
        content_disposition: Option<http::ContentDisposition>,
        content_encoding: Option<http::ContentEncoding>,
        expires: Option<&'a str>,
        content_type: Option<&'a str>,
        forbid_overwrite: Option<bool>,
        encryption: Option<ServerSideEncryption>,
        data_encryption: Option<ServerSideEncryption>,
        encryption_key_id: Option<&'a str>,
        storage_class: Option<StorageClass>,
        oss_tagging: HashMap<String, String>,
    }

    pub struct InitiateMultipartUploadBuilder<'a> {
        client: &'a oss::Client<'a>,
        object: &'a str,
        encoding_type: Option<&'a str>,
        headers: InitiateMultipartUploadBuilderHeaders<'a>,
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
            self.headers.content_type = Some(value);
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

        pub fn with_expires(mut self, value: &'a str) -> Self {
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

        pub fn with_data_encryption(mut self, value: ServerSideEncryption) -> Self {
            self.headers.data_encryption = Some(value);
            self
        }

        pub fn with_encryption_key_id(mut self, value: &'a str) -> Self {
            self.headers.encryption_key_id = Some(value);
            self
        }

        pub fn with_storage_class(mut self, value: StorageClass) -> Self {
            self.headers.storage_class = Some(value);
            self
        }

        pub fn with_oss_tagging(mut self, key: &'a str, value: &'a str) -> Self {
            self.headers
                .oss_tagging
                .insert(key.to_string(), value.to_string());
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
                insert_header(&mut headers, EXPECT, expires);
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
                insert_custom_header(
                    &mut headers,
                    "x-oss-server-side-data-encryption",
                    data_encryption.to_string(),
                );
            }

            if let Some(encryption_key_id) = &self.headers.encryption_key_id {
                insert_custom_header(
                    &mut headers,
                    "x-oss-server-side-encryption-key-id",
                    encryption_key_id,
                );
            }

            if let Some(storage_class) = &self.headers.storage_class {
                insert_custom_header(&mut headers, "x-oss-storage-class", storage_class);
            }

            if !self.headers.oss_tagging.is_empty() {
                let value = serde_qs::to_string(&self.headers.oss_tagging)
                    .expect("Failed to serialize tags");
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
        object: &'a str,
        part_number: u32,
        upload_id: &'a str,
        content: oss::Bytes,
    }

    impl<'a> UploadPartBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client, object: &'a str) -> Self {
            Self {
                client,
                object,
                part_number: Default::default(),
                upload_id: Default::default(),
                content: oss::Bytes::new(),
            }
        }

        pub fn with_part_number(mut self, value: u32) -> Self {
            self.part_number = value;
            self
        }

        pub fn with_upload_id(mut self, value: &'a str) -> Self {
            self.upload_id = value;
            self
        }

        pub fn with_content(mut self, value: oss::Bytes) -> Self {
            self.content = value;
            self
        }

        pub async fn execute(&self) -> api::ApiResult {
            let res = format!(
                "/{}/{}?partNumber={}&uploadId={}",
                self.client.bucket(),
                self.object,
                self.part_number,
                self.upload_id
            );
            let url = format!(
                "{}?partNumber={}&uploadId={}",
                self.client.object_url(self.object),
                self.part_number,
                self.upload_id
            );

            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_method(http::Method::PUT)
                .with_resource(&res)
                .with_body(self.content.to_owned())
                .execute_timeout(self.client.timeout())
                .await?;

            Ok(ApiResponseFrom(resp).to_empty().await)
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

    #[derive(Debug, Default, Serialize)]
    struct CompleteMultipartUploadBuilderQuery<'a> {
        #[serde(rename = "uploadId")]
        upload_id: &'a str,
        #[serde(rename = "encoding-type", skip_serializing_if = "Option::is_none")]
        encoding_type: Option<&'a str>,
    }

    pub struct CompleteMultipartUploadBuilder<'a> {
        client: &'a oss::Client<'a>,
        object: &'a str,
        forbid_overwrite: Option<bool>,
        query: CompleteMultipartUploadBuilderQuery<'a>,
    }

    impl<'a> CompleteMultipartUploadBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client, object: &'a str) -> Self {
            Self {
                client,
                object,
                forbid_overwrite: None,
                query: CompleteMultipartUploadBuilderQuery::default(),
            }
        }

        pub fn with_upload_id(mut self, value: &'a str) -> Self {
            self.query.upload_id = value;
            self
        }

        pub fn with_encoding_type(mut self, value: &'a str) -> Self {
            self.query.encoding_type = Some(value);
            self
        }

        pub fn with_forbid_overwrite(mut self, value: bool) -> Self {
            self.forbid_overwrite = Some(value);
            self
        }

        fn query(&self) -> String {
            serde_qs::to_string(&self.query).unwrap()
        }

        pub async fn execute(&self) -> api::ApiResult<CompleteMultipartUploadResult> {
            let res = format!(
                "/{}/{}?uploadId={}",
                self.client.bucket(),
                &self.object,
                self.query.upload_id
            );
            let url = format!("{}?{}", self.client.object_url(self.object), self.query());

            let mut headers = http::HeaderMap::new();
            insert_header(&mut headers, CONTENT_LENGTH, 0);
            insert_custom_header(&mut headers, "x-oss-complete-all", "yes");
            if let Some(true) = self.forbid_overwrite {
                insert_custom_header(&mut headers, "x-oss-forbid-overwrite", "true");
            }

            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_method(http::Method::POST)
                .with_headers(headers)
                .with_resource(&res)
                .execute()
                .await?;
            Ok(ApiResponseFrom(resp).to_type().await)
        }
    }

    pub struct AbortMultipartUploadBuilder<'a> {
        client: &'a oss::Client<'a>,
        object: &'a str,
        upload_id: &'a str,
    }

    impl<'a> AbortMultipartUploadBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client, object: &'a str) -> Self {
            Self {
                client,
                object,
                upload_id: Default::default(),
            }
        }

        pub fn with_upload_id(mut self, upload_id: &'a str) -> Self {
            self.upload_id = upload_id;
            self
        }

        pub async fn execute(&self) -> api::ApiResult {
            let res = format!(
                "/{}/{}?uploadId={}",
                self.client.bucket(),
                self.object,
                self.upload_id
            );
            let url = format!(
                "{}?uploadId={}",
                self.client.object_url(self.object),
                self.upload_id
            );
            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_method(http::Method::DELETE)
                .with_resource(&res)
                .execute()
                .await?;
            Ok(ApiResponseFrom(resp).to_empty().await)
        }
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    pub(crate) struct ListMultipartUploadsBuilderQuery<'a> {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub(crate) delimiter: Option<&'a str>,
        #[serde(rename = "encoding-type", skip_serializing_if = "Option::is_none")]
        pub(crate) encoding_type: Option<&'a str>,
        #[serde(rename = "key-marker", skip_serializing_if = "Option::is_none")]
        pub(crate) key_marker: Option<&'a str>,
        #[serde(rename = "max-uploads", skip_serializing_if = "Option::is_none")]
        pub(crate) max_uploads: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub(crate) prefix: Option<&'a str>,
        #[serde(rename = "upload-id-marker", skip_serializing_if = "Option::is_none")]
        pub(crate) upload_id_marker: Option<&'a str>,
    }

    pub struct ListMultipartUploadsBuilder<'a> {
        client: &'a oss::Client<'a>,
        query: ListMultipartUploadsBuilderQuery<'a>,
    }

    impl<'a> ListMultipartUploadsBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client) -> Self {
            Self {
                client,
                query: ListMultipartUploadsBuilderQuery::default(),
            }
        }

        pub fn with_delimiter(mut self, value: &'a str) -> Self {
            self.query.delimiter = Some(value);
            self
        }

        pub fn max_uploads(mut self, value: u32) -> Self {
            self.query.max_uploads = Some(value);
            self
        }

        pub fn with_key_marker(mut self, value: &'a str) -> Self {
            self.query.key_marker = Some(value);
            self
        }

        pub fn with_prefix(mut self, value: &'a str) -> Self {
            self.query.prefix = Some(value);
            self
        }

        pub fn with_upload_id_marker(mut self, value: &'a str) -> Self {
            self.query.upload_id_marker = Some(value);
            self
        }

        pub fn with_encoding_type(mut self, value: &'a str) -> Self {
            self.query.encoding_type = Some(value);
            self
        }

        fn query(&self) -> String {
            serde_qs::to_string(&self.query).unwrap()
        }

        pub async fn execute(&self) -> api::ApiResult<ListMultipartUploadsResult> {
            let mut res = format!("/{}/?{}", self.client.bucket(), "uploads");
            let mut url = format!("{}?{}", self.client.base_url(), "uploads");
            let query = self.query();
            if !query.is_empty() {
                res = format!("{}&{}", &res, &query);
                url = format!("{}&{}", &url, &query);
            }
            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_method(http::Method::GET)
                .with_resource(&res)
                .execute_timeout(self.client.timeout())
                .await?;
            Ok(ApiResponseFrom(resp).to_type().await)
        }
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    struct ListPartsBuilderQuery<'a> {
        #[serde(rename = "uploadId")]
        upload_id: &'a str,
        #[serde(rename = "MaxParts", skip_serializing_if = "Option::is_none")]
        max_parts: Option<u64>,
        #[serde(rename = "PartNumberMarker", skip_serializing_if = "Option::is_none")]
        part_number_marker: Option<u64>,
        #[serde(rename = "EncodingType", skip_serializing_if = "Option::is_none")]
        encoding_type: Option<&'a str>,
    }

    pub struct ListPartsBuilder<'a> {
        client: &'a oss::Client<'a>,
        object: &'a str,
        query: ListPartsBuilderQuery<'a>,
    }

    impl<'a> ListPartsBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client, object: &'a str) -> Self {
            Self {
                client,
                object,
                query: ListPartsBuilderQuery::default(),
            }
        }

        pub fn with_upload_id(mut self, value: &'a str) -> Self {
            self.query.upload_id = value;
            self
        }

        pub fn with_max_parts(mut self, value: u64) -> Self {
            self.query.max_parts = Some(value);
            self
        }

        pub fn with_part_number_marker(mut self, value: u64) -> Self {
            self.query.part_number_marker = Some(value);
            self
        }

        pub fn with_encoding_type(mut self, value: &'a str) -> Self {
            self.query.encoding_type = Some(value);
            self
        }

        fn query(&self) -> String {
            serde_qs::to_string(&self.query).unwrap()
        }

        pub async fn execute(&self) -> api::ApiResult<ListPartsResult> {
            let mut res = format!("/{}/{}", self.client.bucket(), self.object);
            let mut url = self.client.object_url(self.object);
            let query = self.query();
            if !query.is_empty() {
                res = format!("{}?{}", res, query);
                url = format!("{}?{}", url, query);
            }
            // dbg!(&res);
            // dbg!(&url);
            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_resource(&res)
                .execute()
                .await?;
            Ok(ApiResponseFrom(resp).to_type().await)
        }
    }
}

/// 基础操作
#[allow(non_snake_case)]
impl<'a> Client<'a> {
    /// 使用Multipart Upload模式传输数据前,您必须先调用InitiateMultipartUpload接口来通知OSS初始化一
    /// 个Multipart Upload事件。
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/initiatemultipartupload)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_object_mutil_init.rs)
    pub fn InitiateMultipartUpload(&self, object: &'a str) -> InitiateMultipartUploadBuilder {
        InitiateMultipartUploadBuilder::new(self, object)
    }

    /// 初始化一个MultipartUpload后,调用UploadPart接口根据指定的Object名和uploadId来分块`Part`上传数据。
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/uploadpart)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_object_mutil_upload_part.rs)
    pub fn UploadPart(&self, object: &'a str) -> UploadPartBuilder {
        UploadPartBuilder::new(self, object)
    }

    /// 通过在UploadPart请求的基础上增加一个请求头x-oss-copy-source来调用UploadPartCopy接口,实现从一个
    /// 已存在的Object中拷贝数据来上传一个Part。
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/uploadpartcopy)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_object_mutil_upload_part.rs)
    pub fn UploadPartCopy(&self) -> UploadPartCopyBuilder {
        UploadPartCopyBuilder::new(self)
    }

    /// 在将所有数据Part都上传完成后,您必须调用CompleteMultipartUpload接口来完成整个文件的分片上传。
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/completemultipartupload)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_object_mutil_comp.rs)
    pub fn CompleteMultipartUpload(&self, object: &'a str) -> CompleteMultipartUploadBuilder {
        CompleteMultipartUploadBuilder::new(self, object)
    }

    /// AbortMultipartUpload接口用于取消MultipartUpload事件并删除对应的Part数据。
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/abortmultipartupload)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_object_mutil_abort.rs)
    pub fn AbortMultipartUpload(&self, object: &'a str) -> AbortMultipartUploadBuilder {
        AbortMultipartUploadBuilder::new(self, object)
    }

    /// 调用ListMultipartUploads接口列举所有执行中的Multipart Upload事件,即已经初始化但还未完成
    /// `Complete`或者还未中止`Abort`的Multipart Upload事件。
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/listmultipartuploads)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_object_mutil_list.rs)
    pub fn ListMultipartUploads(&self) -> ListMultipartUploadsBuilder {
        ListMultipartUploadsBuilder::new(self)
    }

    /// ListParts接口用于列举指定Upload ID所属的所有已经上传成功Part。
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/listparts)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_object_mutil_list_part.rs)
    pub fn ListParts(&self, object: &'a str) -> ListPartsBuilder {
        ListPartsBuilder::new(self, object)
    }
}

#[cfg(test)]
mod tests {
    use super::builders::ListMultipartUploadsBuilderQuery;

    #[test]
    fn list_multipart_uploads_builder_query() {
        let query = ListMultipartUploadsBuilderQuery {
            delimiter: Some("/"),
            max_uploads: Some(32),
            key_marker: Some("abc123"),
            prefix: Some("abc123"),
            upload_id_marker: Some("abc123"),
            encoding_type: Some("url"),
        };
        let q = serde_qs::to_string(&query).unwrap();
        println!("{}", q);
    }
}
