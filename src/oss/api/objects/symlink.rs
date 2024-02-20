use crate::oss;

use self::builders::{GetSymlinkBuilder, PutSymlinkBuilder};

pub mod builders {

    use std::collections::HashMap;

    use chrono::{DateTime, Utc};
    use reqwest::header::{
        CACHE_CONTROL, CONTENT_DISPOSITION, CONTENT_ENCODING, CONTENT_LANGUAGE, CONTENT_TYPE, EXPIRES,
    };

    use crate::oss::{
        self,
        api::{self, insert_custom_header, insert_header, ApiResponseFrom},
        entities::{ObjectACL, StorageClass},
        http,
    };

    #[derive(Debug, Default, Clone)]
    struct PutSymlinkBuilderHeaders {
        cache_control: Option<http::CacheControl>,
        content_disposition: Option<http::ContentDisposition>,
        content_language: Option<String>,
        content_encoding: Option<http::ContentEncoding>,
        content_type: Option<String>,
        expires: Option<DateTime<Utc>>,
    }

    #[derive(Debug)]
    pub struct PutSymlinkBuilder<'a> {
        client: &'a oss::Client<'a>,
        object: &'a str,
        version_id: Option<&'a str>,
        symlink_target: &'a str,
        forbid_overwrite: Option<bool>,
        object_acl: Option<ObjectACL>,
        storage_class: Option<StorageClass>,
        oss_meta: HashMap<&'a str, &'a str>,
        headers: PutSymlinkBuilderHeaders,
    }

    impl<'a> PutSymlinkBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client, object: &'a str) -> Self {
            Self {
                client,
                object,
                version_id: None,
                symlink_target: Default::default(),
                forbid_overwrite: None,
                object_acl: None,
                storage_class: None,
                oss_meta: HashMap::new(),
                headers: PutSymlinkBuilderHeaders::default(),
            }
        }

        pub fn with_symlink_target(mut self, value: &'a str) -> Self {
            self.symlink_target = value;
            self
        }

        pub fn with_forbid_overwrite(mut self, value: bool) -> Self {
            self.forbid_overwrite = Some(value);
            self
        }

        pub fn with_object_acl(mut self, value: ObjectACL) -> Self {
            self.object_acl = Some(value);
            self
        }

        pub fn with_storage_class(mut self, value: StorageClass) -> Self {
            self.storage_class = Some(value);
            self
        }

        pub fn with_oss_meta(mut self, key: &'a str, value: &'a str) -> Self {
            self.oss_meta.insert(key, value);
            self
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

        pub fn with_expires(mut self, value: DateTime<Utc>) -> Self {
            self.headers.expires = Some(value);
            self
        }

        fn headers(&self) -> http::HeaderMap {
            let mut headers = http::HeaderMap::new();
            insert_custom_header(&mut headers, "x-oss-symlink-target", self.symlink_target);
            if let Some(forbid_overwrite) = self.forbid_overwrite {
                insert_custom_header(&mut headers, "x-oss-forbid-overwrite", forbid_overwrite);
            }
            if let Some(object_acl) = &self.object_acl {
                insert_custom_header(&mut headers, "x-oss-object-acl", object_acl.to_string());
            }
            if let Some(storage_class) = &self.storage_class {
                insert_custom_header(&mut headers, "x-oss-storage-class", storage_class.to_string());
            }
            if let Some(content_type) = &self.headers.content_type {
                insert_header(&mut headers, CONTENT_TYPE, content_type);
            }

            if let Some(content_language) = &self.headers.content_language {
                insert_header(&mut headers, CONTENT_LANGUAGE, content_language);
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

            if let Some(expires) = &self.headers.expires {
                insert_header(&mut headers, EXPIRES, expires.format(oss::GMT_DATE_FMT));
            }

            if !self.oss_meta.is_empty() {
                for (key, value) in &self.oss_meta {
                    insert_custom_header(&mut headers, &format!("x-oss-meta-{}", key), value);
                }
            }
            headers
        }

        pub async fn execute(&self) -> api::ApiResult {
            let mut res = format!("/{}/{}?{}", self.client.bucket(), self.object, "symlink");
            let mut url = format!("{}?{}", self.client.object_url(self.object), "symlink");
            if let Some(version_id) = self.version_id {
                res = format!("{}&versionId={}", res, version_id);
                url = format!("{}&versionId={}", url, version_id);
            }

            dbg!(&res);
            dbg!(&url);

            let headers = self.headers();

            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_headers(headers)
                .with_method(http::Method::PUT)
                .with_resource(&res)
                .execute()
                .await?;

            Ok(ApiResponseFrom(resp).to_empty().await)
        }
    }

    pub struct GetSymlinkBuilder<'a> {
        client: &'a oss::Client<'a>,
        object: &'a str,
        version_id: Option<&'a str>,
    }

    impl<'a> GetSymlinkBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client, object: &'a str) -> Self {
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

        pub async fn execute(&self) -> api::ApiResult {
            let mut res = format!("/{}/{}?{}", self.client.bucket(), self.object, "symlink");
            let mut url = { format!("{}?{}", self.client.object_url(self.object), "symlink") };
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
}

/// # 软链接（Symlink）
#[allow(non_snake_case)]
impl<'a> oss::Client<'a> {
    /// 调用PutSymlink接口用于为OSS的目标文件（TargetObject）创建软链接
    /// （Symlink），您可以通过该软链接访问TargetObject。
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/putsymlink)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_object_symlink_put.rs)
    pub fn PutSymlink(&self, object: &'a str) -> PutSymlinkBuilder<'_> {
        PutSymlinkBuilder::new(self, object)
    }

    /// 调用GetSymlink接口获取软链接。此操作需要您对该软链接有读权限。
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/getsymlink)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_object_symlink_get.rs)
    pub fn GetSymlink(&self, object: &'a str) -> GetSymlinkBuilder<'_> {
        GetSymlinkBuilder::new(self, object)
    }
}
