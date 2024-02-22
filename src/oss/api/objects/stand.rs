use builders::{DeleteObjectBuilder, GetObjectBuilder, PutObjectBuilder};

use crate::oss::{self, api::objects::stand::builders::GetObjectMetaBuilder};

use self::builders::{
    AppendObjectBuilder, CopyObjectBuilder, DeleteMultipleObjectsBuilder, HeadObjectBuilder, RestoreObjectBuilder,
};

pub mod builders {

    use std::collections::HashMap;

    use chrono::{DateTime, Utc};
    use oss::http::header::{
        HeaderMap, ACCEPT_ENCODING, CACHE_CONTROL, CONTENT_DISPOSITION, CONTENT_ENCODING, CONTENT_LANGUAGE,
        CONTENT_LENGTH, CONTENT_TYPE, ETAG, EXPIRES, IF_MATCH, IF_MODIFIED_SINCE, IF_NONE_MATCH, IF_UNMODIFIED_SINCE,
        RANGE,
    };
    use serde::{Deserialize, Serialize};

    use crate::oss::{
        self,
        api::{self, insert_custom_header, insert_header, ApiResponseFrom},
        entities::{
            object::{CopyObjectResult, JobParameters, MetadataDirective, RestoreRequest, TaggingDirective, Tier},
            tag::{Tag, TagSet, Tagging},
            ObjectACL, ServerSideEncryption, StorageClass,
        },
        http, Bytes,
    };
    use crate::util::ByteRange;

    #[derive(Debug, Default)]
    struct PutObjectBuilderHeaders<'a> {
        content_type: Option<String>,
        content_encoding: Option<http::ContentEncoding>,
        content_language: Option<String>,
        content_disposition: Option<http::ContentDisposition>,
        cache_control: Option<http::CacheControl>,
        expires: Option<DateTime<Utc>>,
        content_length: Option<u64>,
        content_md5: Option<String>,
        etag: Option<String>,
        forbid_overwrite: Option<bool>,
        encryption: Option<ServerSideEncryption>,
        data_encryption: Option<String>,
        encryption_key_id: Option<String>,
        object_acl: Option<ObjectACL>,
        storage_class: Option<StorageClass>,
        oss_tagging: Option<Vec<(&'a str, &'a str)>>,
        oss_meta: HashMap<String, String>,
    }

    pub struct PutObjectBuilder<'a> {
        client: &'a oss::Client<'a>,
        object: &'a str,
        content: oss::Bytes,
        headers: PutObjectBuilderHeaders<'a>,
        timeout: Option<u64>,
    }

    impl<'a> PutObjectBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client, object: &'a str) -> Self {
            Self {
                client,
                object,
                content: oss::Bytes::new(),
                headers: PutObjectBuilderHeaders::default(),
                timeout: None,
            }
        }

        pub fn with_content_type(mut self, value: &'a str) -> Self {
            self.headers.content_type = Some(value.to_string());
            self
        }

        pub fn with_content_language(mut self, value: &'a str) -> Self {
            self.headers.content_language = Some(value.to_string());
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

        pub fn with_content_md5(mut self, value: &'a str) -> Self {
            self.headers.content_md5 = Some(value.to_string());
            self
        }

        pub fn with_content_length(mut self, value: u64) -> Self {
            self.headers.content_length = Some(value);
            self
        }

        pub fn with_etag(mut self, value: &'a str) -> Self {
            self.headers.etag = Some(value.to_string());
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

        pub fn with_object_acl(mut self, value: ObjectACL) -> Self {
            self.headers.object_acl = Some(value);
            self
        }

        pub fn with_storage_class(mut self, value: StorageClass) -> Self {
            self.headers.storage_class = Some(value);
            self
        }

        pub fn with_oss_tagging(mut self, value: Vec<(&'a str, &'a str)>) -> Self {
            self.headers.oss_tagging = Some(value);
            self
        }

        pub fn with_oss_meta(mut self, key: &'a str, value: &'a str) -> Self {
            self.headers.oss_meta.insert(key.to_string(), value.to_string());
            self
        }

        pub fn with_content(mut self, content: oss::Bytes) -> Self {
            self.content = content;
            self
        }

        pub fn with_timeout(mut self, value: u64) -> Self {
            self.timeout = Some(value);
            self
        }

        fn headers(&self) -> http::HeaderMap {
            let mut headers = http::HeaderMap::new();

            if let Some(content_type) = &self.headers.content_type {
                insert_header(&mut headers, CONTENT_TYPE, content_type);
            }

            if let Some(content_language) = &self.headers.content_language {
                insert_header(&mut headers, CONTENT_LANGUAGE, content_language);
            }
            if let Some(content_type) = &self.headers.content_type {
                insert_header(&mut headers, CONTENT_TYPE, content_type);
            }

            if let Some(cache_control) = &self.headers.cache_control {
                insert_header(&mut headers, CACHE_CONTROL, cache_control);
            }

            if let Some(content_disposition) = &self.headers.content_disposition {
                insert_header(&mut headers, CONTENT_DISPOSITION, content_disposition);
            }

            if let Some(content_encoding) = &self.headers.content_encoding {
                insert_header(&mut headers, CONTENT_ENCODING, content_encoding);
            }

            if let Some(content_length) = &self.headers.content_length {
                insert_header(&mut headers, CONTENT_LENGTH, content_length);
            }

            if let Some(etag) = &self.headers.etag {
                insert_header(&mut headers, ETAG, etag);
            }

            if let Some(content_md5) = &self.headers.content_md5 {
                headers.insert("Content-MD5", content_md5.parse().unwrap());
            }

            if let Some(expires) = &self.headers.expires {
                insert_header(&mut headers, EXPIRES, expires.format(oss::GMT_DATE_FMT));
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

            if let Some(object_acl) = &self.headers.object_acl {
                insert_custom_header(&mut headers, "x-oss-object-acl", object_acl);
            }

            if let Some(storage_class) = &self.headers.storage_class {
                insert_custom_header(&mut headers, "x-oss-storage-class", storage_class);
            }

            if let Some(tags) = &self.headers.oss_tagging {
                let kv: HashMap<&str, &str> = tags.to_owned().into_iter().collect();
                let value = serde_qs::to_string(&kv).unwrap();
                insert_custom_header(&mut headers, "x-oss-tagging", value);
            }

            if !self.headers.oss_meta.is_empty() {
                for (key, value) in &self.headers.oss_meta {
                    insert_custom_header(&mut headers, &format!("x-oss-meta-{}", key), value);
                }
            }
            headers
        }

        pub async fn execute(&self) -> api::ApiResult<()> {
            let res = format!("/{}/{}", self.client.bucket(), self.object);
            let url = self.client.object_url(self.object);
            let headers = self.headers();

            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_method(http::Method::PUT)
                .with_headers(headers)
                .with_body(self.content.to_owned())
                .with_resource(&res)
                .execute_timeout(self.timeout.unwrap_or(self.client.timeout()))
                .await?;
            Ok(ApiResponseFrom(resp).to_empty().await)
        }
    }

    #[derive(Debug, Default, Clone)]
    struct CopyObjectBuilderArguments<'a> {
        copy_source: Option<&'a str>,
        source_version_id: Option<&'a str>,
        version_id: Option<&'a str>,
        forbid_overwrite: Option<bool>,
        if_match: Option<&'a str>,
        if_none_match: Option<&'a str>,
        if_unmodified_since: Option<DateTime<Utc>>,
        if_modified_since: Option<DateTime<Utc>>,
        metadata_directive: Option<MetadataDirective>,
        encryption: Option<ServerSideEncryption>,
        enc_key_id: Option<&'a str>,
        object_acl: Option<ObjectACL>,
        storage_class: Option<StorageClass>,
        // oss_tagging: Option<Tagging>,
        oss_tagging: Option<Vec<(&'a str, &'a str)>>,
        tagging_directive: Option<TaggingDirective>,
    }

    #[derive(Debug, Clone)]
    pub struct CopyObjectBuilder<'a> {
        client: &'a oss::Client<'a>,
        object: &'a str,
        arguments: CopyObjectBuilderArguments<'a>,
    }

    impl<'a> CopyObjectBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client, object: &'a str) -> Self {
            Self {
                client,
                object,
                arguments: CopyObjectBuilderArguments::default(),
            }
        }

        /// 指定CopyObject操作时是否覆盖同名目标Object。当目标Bucket处于已开
        /// 启或已暂停版本控制状态时,x-oss-forbid-overwrite请求Header设置
        /// 无效,即允许覆盖同名Object。
        pub fn with_forbid_overwrite(mut self, value: bool) -> Self {
            self.arguments.forbid_overwrite = Some(value);
            self
        }

        /// 指定拷贝的源地址
        pub fn with_copy_source(mut self, value: &'a str) -> Self {
            self.arguments.copy_source = Some(value);
            self
        }

        pub fn with_source_version_id(mut self, value: &'a str) -> Self {
            self.arguments.source_version_id = Some(value);
            self
        }

        pub fn with_version_id(mut self, value: &'a str) -> Self {
            self.arguments.version_id = Some(value);
            self
        }

        /// 如果源Object的ETag值和您提供的ETag相等,则执行拷贝操作,并返回200 OK
        pub fn with_if_match(mut self, value: &'a str) -> Self {
            self.arguments.if_match = Some(value);
            self
        }

        pub fn with_if_none_match(mut self, value: &'a str) -> Self {
            self.arguments.if_none_match = Some(value);
            self
        }

        pub fn with_if_unmodified_since(mut self, value: DateTime<Utc>) -> Self {
            self.arguments.if_unmodified_since = Some(value);
            self
        }

        pub fn with_if_modified_since(mut self, value: DateTime<Utc>) -> Self {
            self.arguments.if_modified_since = Some(value);
            self
        }

        pub fn with_metadata_directive(mut self, value: MetadataDirective) -> Self {
            self.arguments.metadata_directive = Some(value);
            self
        }

        pub fn with_encryption(mut self, value: ServerSideEncryption) -> Self {
            self.arguments.encryption = Some(value);
            self
        }

        pub fn with_enc_key_id(mut self, value: &'a str) -> Self {
            self.arguments.enc_key_id = Some(value);
            self
        }

        pub fn with_object_acl(mut self, value: ObjectACL) -> Self {
            self.arguments.object_acl = Some(value);
            self
        }

        pub fn with_storage_class(mut self, value: StorageClass) -> Self {
            self.arguments.storage_class = Some(value);
            self
        }

        pub fn with_oss_tagging(mut self, value: Vec<(&'a str, &'a str)>) -> Self {
            self.arguments.oss_tagging = Some(value);
            self
        }

        pub fn with_tagging_directive(mut self, value: TaggingDirective) -> Self {
            self.arguments.tagging_directive = Some(value);
            self
        }

        fn headers(&self) -> HeaderMap {
            let mut headers = HeaderMap::new();
            if let Some(true) = self.arguments.forbid_overwrite {
                insert_custom_header(&mut headers, "x-oss-forbid-overwrite", "true");
            }

            if let Some(copy_source) = self.arguments.copy_source {
                let value = if let Some(source_version_id) = self.arguments.source_version_id {
                    format!("{}?versionId={}", copy_source, source_version_id)
                } else {
                    copy_source.to_string()
                };
                let key = "x-oss-copy-source";
                insert_custom_header(&mut headers, key, value);
            }

            if let Some(value) = self.arguments.if_match {
                let key = "x-oss-copy-source-if-match";
                insert_custom_header(&mut headers, key, value);
            }

            if let Some(value) = self.arguments.if_none_match {
                let key = "x-oss-copy-source-if-none-match";
                insert_custom_header(&mut headers, key, value);
            }

            if let Some(value) = &self.arguments.if_unmodified_since {
                let key = "x-oss-copy-source-if-unmodified-since";
                insert_custom_header(&mut headers, key, value.format(oss::GMT_DATE_FMT).to_string())
            }

            if let Some(value) = &self.arguments.if_modified_since {
                let key = "x-oss-copy-source-if-modified-since";
                insert_custom_header(&mut headers, key, value.format(oss::GMT_DATE_FMT).to_string())
            }

            if let Some(value) = &self.arguments.metadata_directive {
                let key = "x-oss-metadata-directive";
                insert_custom_header(&mut headers, key, value.to_string())
            }

            if let Some(value) = &self.arguments.encryption {
                let key = "x-oss-server-side-encryption";
                insert_custom_header(&mut headers, key, value.to_string())
            }

            if let Some(value) = self.arguments.enc_key_id {
                let key = "x-oss-server-side-encryption-key-id";
                insert_custom_header(&mut headers, key, value)
            }

            if let Some(value) = &self.arguments.object_acl {
                let key = "x-oss-object-acl";
                insert_custom_header(&mut headers, key, value.to_string())
            }

            if let Some(value) = &self.arguments.storage_class {
                let key = "x-oss-storage-class";
                insert_custom_header(&mut headers, key, value.to_string())
            }

            if let Some(tags) = &self.arguments.oss_tagging {
                let kv: HashMap<&str, &str> = tags.to_owned().into_iter().collect();
                let value = serde_qs::to_string(&kv).unwrap();
                insert_custom_header(&mut headers, "x-oss-tagging", value);
            }

            if let Some(value) = &self.arguments.tagging_directive {
                let key = "x-oss-tagging-directive";
                insert_custom_header(&mut headers, key, value.to_string())
            }

            headers
        }

        pub async fn execute(&self) -> api::ApiResult<CopyObjectResult> {
            let res = format!("/{}/{}", self.client.bucket(), self.object);
            let url = self.client.object_url(self.object);
            let headers = self.headers();
            dbg!(&headers);
            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_method(http::Method::PUT)
                .with_headers(headers)
                .with_resource(&res)
                .execute_timeout(self.client.timeout())
                .await?;
            Ok(ApiResponseFrom(resp).to_type().await)
        }
    }

    #[derive(Debug, Default)]
    struct AppendObjectBuilderArguments {
        cache_control: Option<String>,
        content_disposition: Option<String>,
        content_encoding: Option<String>,
        // content_md5: Option<String>,
        expires: Option<DateTime<Utc>>,
        server_side_encryption: Option<ServerSideEncryption>,
        object_acl: Option<ObjectACL>,
        storage_class: Option<StorageClass>,
        // meta: Option<Vec<String>>,
        // tagging: Option<Tagging>,
    }

    pub struct AppendObjectBuilder<'a> {
        client: &'a oss::Client<'a>,
        object: String,
        position: u64,
        arguments: AppendObjectBuilderArguments,
    }

    impl<'a> AppendObjectBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client, object: &'a str) -> Self {
            Self {
                client,
                object: object.to_string(),
                position: 0,
                arguments: AppendObjectBuilderArguments::default(),
            }
        }

        pub fn position(mut self, value: u64) -> Self {
            self.position = value;
            self
        }

        pub fn cache_control(mut self, value: &'a str) -> Self {
            self.arguments.cache_control = Some(value.to_string());
            self
        }
        pub fn content_disposition(mut self, value: &'a str) -> Self {
            self.arguments.content_disposition = Some(value.to_string());
            self
        }
        pub fn content_encoding(mut self, value: &str) -> Self {
            self.arguments.content_encoding = Some(value.to_string());
            self
        }

        pub fn expires(mut self, value: DateTime<Utc>) -> Self {
            self.arguments.expires = Some(value);
            self
        }

        pub fn server_side_encryption(mut self, value: ServerSideEncryption) -> Self {
            self.arguments.server_side_encryption = Some(value);
            self
        }

        pub fn object_acl(mut self, value: ObjectACL) -> Self {
            self.arguments.object_acl = Some(value);
            self
        }

        pub fn storage_class(mut self, value: StorageClass) -> Self {
            self.arguments.storage_class = Some(value);
            self
        }

        // pub fn metas(mut self) -> Self {
        //   self
        // }

        // pub fn add_meta(mut self) -> Self {
        //   self
        // }

        // pub fn tagging(mut self) -> Self {
        //   self
        // }

        // pub fn add_tag(mut self) -> Self {
        //   self
        // }

        pub async fn execute(&self) -> api::ApiResult<()> {
            self.client;
            let _ = self.object;
            todo!()
        }
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    pub(crate) struct GetObjectBuilderQuery<'a> {
        #[serde(rename = "response-cache-control", skip_serializing_if = "Option::is_none")]
        cache_control: Option<&'a str>,
        #[serde(rename = "response-content-disposition", skip_serializing_if = "Option::is_none")]
        content_disposition: Option<&'a str>,
        #[serde(rename = "response-content-encoding", skip_serializing_if = "Option::is_none")]
        content_encoding: Option<&'a str>,
        #[serde(rename = "response-content-language", skip_serializing_if = "Option::is_none")]
        content_language: Option<&'a str>,
        #[serde(rename = "response-content-type", skip_serializing_if = "Option::is_none")]
        content_type: Option<&'a str>,
        #[serde(rename = "response-expires", skip_serializing_if = "Option::is_none")]
        expires: Option<&'a str>,
        #[serde(rename = "versionId", skip_serializing_if = "Option::is_none")]
        version_id: Option<&'a str>,
    }

    #[derive(Debug)]
    pub struct GetObjectBuilder<'a> {
        client: &'a oss::Client<'a>,
        object: &'a str,
        range: Option<ByteRange>,
        modified_since: Option<DateTime<Utc>>,
        unmodified_since: Option<DateTime<Utc>>,
        r#match: Option<&'a str>,
        none_match: Option<&'a str>,
        accept_encoding: Option<&'a str>,
        query: GetObjectBuilderQuery<'a>,
        timeout: Option<u64>,
    }

    impl<'a> GetObjectBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client, object: &'a str) -> Self {
            Self {
                client,
                object,
                range: None,
                r#match: None,
                modified_since: None,
                unmodified_since: None,
                none_match: None,
                accept_encoding: None,
                query: GetObjectBuilderQuery::default(),
                timeout: None,
            }
        }

        pub fn with_version_id(mut self, value: &'a str) -> Self {
            self.query.version_id = Some(value);
            self
        }

        pub fn with_content_type(mut self, value: &'a str) -> Self {
            self.query.content_type = Some(value);
            self
        }

        pub fn with_content_language(mut self, value: &'a str) -> Self {
            self.query.content_language = Some(value);
            self
        }

        pub fn with_expires(mut self, value: &'a str) -> Self {
            self.query.expires = Some(value);
            self
        }

        pub fn with_cache_control(mut self, value: &'a str) -> Self {
            self.query.cache_control = Some(value);
            self
        }

        pub fn with_content_disposition(mut self, value: &'a str) -> Self {
            self.query.content_disposition = Some(value);
            self
        }

        pub fn with_content_encoding(mut self, value: &'a str) -> Self {
            self.query.content_encoding = Some(value);
            self
        }

        pub fn with_range(mut self, value: ByteRange) -> Self {
            self.range = Some(value);
            self
        }

        pub fn with_modified_since(mut self, value: DateTime<Utc>) -> Self {
            self.modified_since = Some(value);
            self
        }

        pub fn with_unmodified_since(mut self, value: DateTime<Utc>) -> Self {
            self.unmodified_since = Some(value);
            self
        }

        pub fn with_match(mut self, value: &'a str) -> Self {
            self.r#match = Some(value);
            self
        }

        pub fn with_none_match(mut self, value: &'a str) -> Self {
            self.none_match = Some(value);
            self
        }

        pub fn with_accept_encoding(mut self, value: &'a str) -> Self {
            self.accept_encoding = Some(value);
            self
        }

        pub fn with_timeout(mut self, value: u64) -> Self {
            self.timeout = Some(value);
            self
        }

        pub(crate) fn query(&self) -> String {
            serde_qs::to_string(&self.query).unwrap()
        }

        pub(crate) fn headers(&self) -> http::HeaderMap {
            let mut headers = http::HeaderMap::new();
            if let Some(range) = &self.range {
                insert_header(&mut headers, RANGE, range.to_string());
            }
            if let Some(modified_since) = &self.modified_since {
                insert_header(&mut headers, IF_MODIFIED_SINCE, modified_since);
            }
            if let Some(unmodified_since) = &self.unmodified_since {
                let dt = unmodified_since.format(oss::GMT_DATE_FMT).to_string();
                insert_header(&mut headers, IF_UNMODIFIED_SINCE, dt);
            }
            if let Some(r#match) = &self.r#match {
                insert_header(&mut headers, IF_MATCH, r#match);
            }
            if let Some(none_match) = &self.none_match {
                insert_header(&mut headers, IF_NONE_MATCH, none_match);
            }
            if let Some(accept_encoding) = &self.accept_encoding {
                insert_header(&mut headers, ACCEPT_ENCODING, accept_encoding);
            }
            headers
        }

        pub async fn execute(&self) -> api::ApiResult<Bytes> {
            let mut res = format!("/{}/{}", self.client.bucket(), self.object);
            let mut url = self.client.object_url(self.object);
            let query = self.query();
            // dbg!(&query);
            if !query.is_empty() {
                res = format!("{}?{}", res, query);
                url = format!("{}?{}", url, query)
            }

            let headers = self.headers();
            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_headers(headers)
                .with_resource(&res)
                .execute_timeout(self.timeout.unwrap_or(self.client.timeout()))
                .await?;

            Ok(ApiResponseFrom(resp).to_bytes().await)
        }
    }

    #[derive(Debug)]
    pub struct DeleteObjectBuilder<'a> {
        client: &'a oss::Client<'a>,
        object: &'a str,
        version_id: Option<&'a str>,
    }

    impl<'a> DeleteObjectBuilder<'a> {
        pub fn new(client: &'a oss::Client, object: &'a str) -> Self {
            Self {
                client,
                object,
                version_id: None,
            }
        }

        pub fn with_version_id(mut self, value: &'a str) -> Self {
            self.version_id = Some(value);
            self
        }

        pub async fn execute(&self) -> api::ApiResult<()> {
            let mut res = format!("/{}/{}", self.client.bucket(), self.object);
            let mut url = self.client.object_url(self.object);
            if let Some(version_id) = self.version_id {
                res = format!("{}?versionId={}", res, version_id);
                url = format!("{}?versionId={}", url, version_id);
            }

            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_resource(&res)
                .with_method(http::Method::DELETE)
                .execute()
                .await?;

            Ok(ApiResponseFrom(resp).to_empty().await)
        }
    }

    #[allow(unused)]
    pub struct DeleteMultipleObjectsBuilder<'a> {
        client: &'a oss::Client<'a>,
        quiet: Option<bool>,
        encoding_type: Option<&'a str>,
        content_length: Option<u64>,
        content_md5: Option<&'a str>,
    }

    impl<'a> DeleteMultipleObjectsBuilder<'a> {
        pub fn new(client: &'a oss::Client) -> Self {
            Self {
                client,
                quiet: None,
                encoding_type: None,
                content_length: None,
                content_md5: None,
            }
        }

        pub fn with_quiet(mut self, value: bool) -> Self {
            self.quiet = Some(value);
            self
        }

        pub fn with_encoding_type(mut self, value: &'a str) -> Self {
            self.encoding_type = Some(value);
            self
        }

        pub async fn execute(&self) -> api::ApiResult {
            let res = format!("/{}/?{}", self.client.bucket(), "delete");
            let url = format!("{}/?{}", self.client.base_url(), "delete");
            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_method(http::Method::POST)
                .with_resource(&res)
                .execute_timeout(self.client.timeout())
                .await?;
            Ok(ApiResponseFrom(resp).to_empty().await)
        }
    }

    pub struct HeadObjectBuilder<'a> {
        client: &'a oss::Client<'a>,
        object: &'a str,
        version_id: Option<&'a str>,
        modified_since: Option<DateTime<Utc>>,
        unmodified_since: Option<DateTime<Utc>>,
        r#match: Option<&'a str>,
        none_match: Option<&'a str>,
    }

    impl<'a> HeadObjectBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client, object: &'a str) -> Self {
            Self {
                client,
                object,
                version_id: None,
                modified_since: None,
                unmodified_since: None,
                r#match: None,
                none_match: None,
            }
        }

        pub fn with_version_id(mut self, version_id: &'a str) -> Self {
            self.version_id = Some(version_id);
            self
        }

        pub fn with_modified_since(mut self, value: DateTime<Utc>) -> Self {
            self.modified_since = Some(value);
            self
        }

        pub fn with_unmodified_since(mut self, value: DateTime<Utc>) -> Self {
            self.unmodified_since = Some(value);
            self
        }

        pub fn with_match(mut self, value: &'a str) -> Self {
            self.r#match = Some(value);
            self
        }

        pub fn with_none_match(mut self, value: &'a str) -> Self {
            self.none_match = Some(value);
            self
        }

        fn headers(&self) -> http::HeaderMap {
            let mut headers = http::HeaderMap::new();
            if let Some(modified_since) = self.modified_since {
                insert_header(
                    &mut headers,
                    IF_MODIFIED_SINCE,
                    modified_since.format(oss::GMT_DATE_FMT).to_string(),
                );
            }

            if let Some(unmodified_since) = self.unmodified_since {
                insert_header(
                    &mut headers,
                    IF_UNMODIFIED_SINCE,
                    unmodified_since.format(oss::GMT_DATE_FMT).to_string(),
                );
            }
            if let Some(r#match) = self.r#match {
                insert_header(&mut headers, IF_MATCH, r#match);
            }
            if let Some(none_match) = self.none_match {
                insert_header(&mut headers, IF_NONE_MATCH, none_match);
            }
            headers
        }

        pub async fn execute(&self) -> api::ApiResult {
            let mut res = format!("/{}/{}", self.client.bucket(), self.object);
            let mut url = self.client.object_url(self.object);
            if let Some(version_id) = self.version_id {
                res = format!("{}?versionId={}", res, version_id);
                url = format!("{}?versionId={}", url, version_id);
            };

            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_method(http::Method::HEAD)
                .with_headers(self.headers())
                .with_resource(&res)
                .execute_timeout(self.client.timeout())
                .await?;

            Ok(ApiResponseFrom(resp).to_empty().await)
        }
    }

    pub struct GetObjectMetaBuilder<'a> {
        client: &'a oss::Client<'a>,
        object: &'a str,
        version_id: Option<&'a str>,
    }

    impl<'a> GetObjectMetaBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client, object: &'a str) -> Self {
            Self {
                client,
                object,
                version_id: None,
            }
        }

        pub fn with_version_id(mut self, version_id: &'a str) -> Self {
            self.version_id = Some(version_id);
            self
        }

        pub async fn execute(&self) -> api::ApiResult<()> {
            let mut res = format!("/{}/{}?{}", self.client.options.bucket, self.object, "objectMeta");

            let mut url = format!("{}?{}", self.client.object_url(self.object), "objectMeta");

            if let Some(version_id) = self.version_id {
                res = format!("{}&versionId={}", res, version_id);
                url = format!("{}&versionId={}", url, version_id);
            }

            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_resource(&res)
                .execute()
                .await?;

            Ok(ApiResponseFrom(resp).to_empty().await)
        }
    }

    pub struct RestoreObjectBuilder<'a> {
        client: &'a oss::Client<'a>,
        object: &'a str,
        version_id: Option<&'a str>,
        days: Option<u8>,
        tier: Option<Tier>,
    }

    impl<'a> RestoreObjectBuilder<'a> {
        pub fn new(client: &'a oss::Client, object: &'a str) -> Self {
            Self {
                client,
                object,
                version_id: None,
                days: None,
                tier: None,
            }
        }

        pub fn with_days(mut self, days: u8) -> Self {
            self.days = Some(days);
            self
        }

        pub fn with_tier(mut self, tier: Tier) -> Self {
            self.tier = Some(tier);
            self
        }

        fn config(&self) -> Option<String> {
            let days = self.days?;
            let request = RestoreRequest {
                days,
                job_parameters: self.tier.as_ref().map(|tier| JobParameters { tier: tier.clone() }),
            };
            quick_xml::se::to_string(&request).ok()
        }

        pub async fn execute(&self) -> api::ApiResult<()> {
            let mut res = format!("/{}/{}?{}", self.client.bucket(), self.object, "restore");
            let mut url = format!("{}?{}", self.client.object_url(self.object), "restore");
            if let Some(version_id) = self.version_id {
                res = format!("{}&versionId={}", res, version_id);
                url = format!("{}&versionId={}", url, version_id);
            };

            let config = Bytes::from(self.config().unwrap_or("".to_string()));

            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_method(http::Method::POST)
                .with_body(config)
                .with_resource(&res)
                .execute()
                .await?;

            Ok(ApiResponseFrom(resp).to_empty().await)
        }
    }
}

/// # 基础操作
#[allow(non_snake_case)]
impl<'a> oss::Client<'a> {
    /// 调用PutObject接口上传文件（Object）
    ///
    /// - [official docs]()
    /// - [xtoss example]()
    pub fn PutObject(&self, object: &'a str) -> PutObjectBuilder {
        PutObjectBuilder::new(self, object)
    }

    /// GetObject接口用于获取某个文件（Object）。此操作需要对此Object具有读权限
    ///
    /// - [official docs]()
    /// - [xtoss example]()
    pub fn GetObject(&self, object: &'a str) -> GetObjectBuilder {
        GetObjectBuilder::new(self, object)
    }

    /// 调用CopyObject接口拷贝同一地域下相同或不同存储空间（Bucket）之间的文件（Object）
    ///
    /// - [official docs]()
    /// - [xtoss example]()
    pub fn CopyObject(&self, object: &'a str) -> CopyObjectBuilder {
        CopyObjectBuilder::new(self, object)
    }

    /// 调用AppendObject接口用于以追加写的方式上传文件（Object）。通过AppendObject操
    ///
    /// - [official docs]()
    /// - [xtoss example]()
    pub fn AppendObject(&self, object: &'a str) -> AppendObjectBuilder {
        AppendObjectBuilder::new(self, object)
    }

    /// 调用DeleteObject删除某个文件（Object）
    ///
    /// - [official docs]()
    /// - [xtoss example]()
    pub fn DeleteObject(&self, object: &'a str) -> DeleteObjectBuilder {
        DeleteObjectBuilder::new(self, object)
    }

    /// DeleteMultipleObjects接口用于删除同一个存储空间（Bucket）中的多个文件（Object）
    /// - [official docs]()
    /// - [xtoss example]()
    pub fn DeleteMultipleObjects(&self) -> DeleteMultipleObjectsBuilder {
        DeleteMultipleObjectsBuilder::new(self)
    }

    /// HeadObject接口用于获取某个文件（Object）的元信息
    ///
    /// - [official docs]()
    /// - [xtoss example]()
    pub fn HeadObject(&self, object: &'a str) -> HeadObjectBuilder {
        HeadObjectBuilder::new(self, object)
    }

    /// 调用GetObjectMeta接口获取一个文件（Object）的元数据信息
    /// 包括该Object的ETag、Size、LastModified信息，并且不返回该Object的内容。
    ///
    /// - [official docs]()
    /// - [xtoss example]()
    pub fn GetObjectMeta(&self, object: &'a str) -> GetObjectMetaBuilder {
        GetObjectMetaBuilder::new(self, object)
    }

    /// 调用RestoreObject接口解冻归档类型、冷归档、深度冷归档类型的文件（Object）
    ///
    /// - [official docs]()
    /// - [xtoss example]()
    pub fn RestoreObject(&self, object: &'a str) -> RestoreObjectBuilder {
        RestoreObjectBuilder::new(self, object)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::oss::{self, http::ContentDisposition};
    use crate::util::ByteRange;
    use chrono::Utc;
    #[test]
    fn get_object_builder_arugments() {
        let option = oss::Options::default();
        let client = oss::Client::new(option);
        let filename = Some("测试.txt".to_string());
        let content_disposition = ContentDisposition::ATTACHMENT(filename).to_string();

        let builder = GetObjectBuilder::new(&client, "example/ex1.txt")
            .with_version_id("version123")
            .with_content_type("text/plain")
            .with_content_language("zh-CN")
            .with_expires("expires")
            .with_cache_control("cache")
            .with_content_disposition(content_disposition.as_str())
            .with_content_encoding("GZIP")
            .with_range(ByteRange::new().with_start(500).with_amount(1000))
            .with_modified_since(Utc::now())
            .with_unmodified_since(Utc::now())
            .with_match("etag")
            .with_none_match("etag")
            .with_accept_encoding("text/plain");

        let left = r#"response-cache-control=cache&response-content-disposition=attachment%3Bfilename%3D%22%E6%B5%8B%E8%AF%95.txt%22&response-content-encoding=GZIP&response-content-language=zh-CN&response-content-type=text%2Fplain&response-expires=expires&versionId=version123"#;

        assert_eq!(left, builder.query());
    }
}
