use crate::oss::{self, api::objects::stand::builder::DeleteObjectBuilder};

use builder::PutObjectBuilder;

use self::builder::GetObjectBuilder;

pub mod builder {
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};
    use urlencoding;

    use crate::oss::{
        self,
        entities::{tag::Tagging, ObjectACL, ServerSideEncryption, StorageClass},
        header::{self, HeaderMap},
        Bytes,
    };

    pub struct PutObjectBuilder<'a> {
        client: &'a oss::Client<'a>,
        object: &'a str,
        headers: Option<HeaderMap>,
        content: oss::Bytes,
    }

    #[allow(unused)]
    impl<'a> PutObjectBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client, object: &'a str) -> Self {
            Self {
                client,
                object,
                content: oss::Bytes::new(),
                headers: None,
            }
        }

        pub fn with_content(mut self, content: oss::Bytes) -> Self {
            self.content = content;
            self
        }

        pub fn with_headers(mut self, headers: HeaderMap) -> Self {
            self.headers = Some(headers);
            self
        }

        pub async fn send(&self) -> oss::Result<()> {
            let url = format!("{}/{}", self.client.options.base_url(), self.object);

            let mut task = self
                .client
                .request
                .task()
                .url(&url)
                .method(oss::Method::PUT)
                .body(self.content.to_owned());

            let task = match &self.headers {
                Some(headers) => task.headers(headers.clone()),
                None => task,
            };

            let resp = task.send().await?;

            let result = oss::Data {
                status: resp.status,
                headers: resp.headers,
                data: (),
            };
            Ok(result)
        }
    }

    #[allow(unused)]
    pub struct AppendObjectBuilder<'a> {
        client: &'a oss::Client<'a>,
        object: String,
        position: u64,
        cache_control: Option<String>,
        content_disposition: Option<String>,
        content_encoding: Option<String>,
        content_md5: Option<String>,
        expires: Option<DateTime<Utc>>,
        server_side_encryption: Option<ServerSideEncryption>,
        object_acl: Option<ObjectACL>,
        storage_class: Option<StorageClass>,
        meta: Option<Vec<String>>,
        tagging: Option<Tagging>,
    }

    #[allow(unused)]
    impl<'a> AppendObjectBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client, object: &'a str) -> Self {
            Self {
                client,
                object: object.to_string(),
                position: 0,
                cache_control: None,
                content_disposition: None,
                content_encoding: None,
                content_md5: None,
                expires: None,
                server_side_encryption: None,
                object_acl: None,
                storage_class: None,
                meta: None,
                tagging: None,
            }
        }

        pub fn position(mut self, value: u64) -> Self {
            self.position = value;
            self
        }

        pub fn cache_control(mut self, value: &'a str) -> Self {
            self.cache_control = Some(value.to_string());
            self
        }
        pub fn content_disposition(mut self, value: &'a str) -> Self {
            self.content_disposition = Some(value.to_string());
            self
        }
        pub fn content_encoding(mut self, value: &str) -> Self {
            self.content_encoding = Some(value.to_string());
            self
        }

        pub fn expires(mut self, value: DateTime<Utc>) -> Self {
            self.expires = Some(value);
            self
        }

        pub fn server_side_encryption(mut self, value: ServerSideEncryption) -> Self {
            self.server_side_encryption = Some(value);
            self
        }

        pub fn object_acl(mut self, value: ObjectACL) -> Self {
            self.object_acl = Some(value);
            self
        }

        pub fn storage_class(mut self, value: StorageClass) -> Self {
            self.storage_class = Some(value);
            self
        }

        pub fn metas(mut self) -> Self {
            self
        }

        pub fn add_meta(mut self) -> Self {
            self
        }

        pub fn tagging(mut self) -> Self {
            self
        }

        pub fn add_tag(mut self) -> Self {
            self
        }
    }

    #[allow(unused)]
    #[derive(Debug, Default, Serialize, Deserialize)]
    pub struct GetObjectBuilderQuery<'a> {
        #[serde(
            rename = "response-cache-control",
            skip_serializing_if = "Option::is_none"
        )]
        cache_control: Option<&'a str>,
        #[serde(
            rename = "response-content-disposition",
            skip_serializing_if = "Option::is_none"
        )]
        content_disposition: Option<&'a str>,
        #[serde(
            rename = "response-content-encoding",
            skip_serializing_if = "Option::is_none"
        )]
        content_encoding: Option<&'a str>,
        #[serde(
            rename = "response-content-language",
            skip_serializing_if = "Option::is_none"
        )]
        content_language: Option<&'a str>,
        #[serde(
            rename = "response-content-type",
            skip_serializing_if = "Option::is_none"
        )]
        content_type: Option<&'a str>,
        #[serde(rename = "response-expires", skip_serializing_if = "Option::is_none")]
        expires: Option<&'a str>,
        #[serde(rename = "versionId", skip_serializing_if = "Option::is_none")]
        version_id: Option<&'a str>,
    }

    // #[allow(unused)]
    // #[derive(Debug, Default)]
    // pub struct GetObjectBuilderQuery2<'a> {
    //     version_id: Option<&'a str>,
    //     content_encoding: Option<&'a str>,
    //     content_type: Option<&'a str>,
    //     content_language: Option<&'a str>,
    //     expires: Option<&'a str>,
    //     cache_control: Option<&'a str>,
    //     content_disposition: Option<&'a str>,
    // }

    // #[allow(unused)]
    // impl<'a> GetObjectBuilderQuery2<'a> {
    //     pub fn to_query() -> String {
    //         "".to_string()
    //     }
    // }

    #[allow(unused)]
    #[derive(Debug)]
    pub struct GetObjectBuilder<'a> {
        client: &'a oss::Client<'a>,
        object: &'a str,
        range: Option<(u64, u64)>,
        modified_since: Option<DateTime<Utc>>,
        unmodified_since: Option<DateTime<Utc>>,
        r#match: Option<&'a str>,
        none_match: Option<&'a str>,
        accept_encoding: Option<&'a str>,
        query: GetObjectBuilderQuery<'a>,
    }

    #[allow(unused)]
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

        pub fn with_range(mut self, value: (u64, u64)) -> Self {
            self.range = Some(value);
            self
        }

        /// 如果指定的时间早于实际修改时间或指定的时间不符合规范，则直接返回Object，
        /// 并返回200 OK；如果指定的时间等于或者晚于实际修改时间，则返回304 Not Modified。
        ///
        /// 时间格式：GMT，例如Fri, 13 Nov 2015 14:47:53 GMT
        ///
        /// 默认值：无
        pub fn with_modified_since(mut self, value: DateTime<Utc>) -> Self {
            self.modified_since = Some(value);
            self
        }

        /// 如果指定的时间等于或者晚于Object实际修改时间，则正常传输Object，并返回200 OK；
        /// 如果指定的时间早于实际修改时间，则返回412 Precondition Failed。
        ///
        /// 时间格式：GMT，例如Fri, 13 Nov 2015 14:47:53 GMTIf-Modified-Since和
        /// If-Unmodified-Since可以同时使用。
        ///
        /// 默认值：无
        pub fn with_unmodified_since(mut self, value: DateTime<Utc>) -> Self {
            self.unmodified_since = Some(value);
            self
        }

        /// ### 设置 If-Match
        ///
        /// 如果传入的ETag和Object的ETag匹配，则正常传输Object，并返回200 OK；
        /// 如果传入的ETag和Object的ETag不匹配，则返回412 Precondition Failed。
        ///
        /// Object的ETag值用于验证数据是否发生了更改，您可以基于ETag值验证数据完整性。
        ///
        /// 默认值：无
        pub fn with_match(mut self, value: &'a str) -> Self {
            self.r#match = Some(value);
            self
        }

        /// ### 设置 If-None-Match
        ///
        /// 如果传入的ETag值和Object的ETag不匹配，则正常传输Object，并返回200 OK；
        ///
        /// 如果传入的ETag和Object的ETag匹配，则返回304 Not Modified。
        /// `If-Match`和`If-None-Match`可以同时使用。
        ///
        /// 默认值：无
        pub fn with_none_match(mut self, value: &'a str) -> Self {
            self.none_match = Some(value);
            self
        }

        /// ### 指定客户端的编码类型。
        ///
        /// 如果要对返回内容进行Gzip压缩传输，您需要在请求头中以显示方式加入Accept-Encoding:gzip
        ///  OSS会根据Object的Content-Type和Object大小（不小于1 KB）判断是否返回经过Gzip压缩
        /// 的数据。
        /// 1. 如果采用了Gzip压缩，则不会附带ETag信息。
        /// 2. 目前OSS支持Gzip压缩的`Content-Type`为`text/cache-manifest`、 `text/xml`、`text/plain`、`text/css`、`application/javascript`、`application/x-javascript`、`application/rss+xml`、`application/json和text/json`。
        pub fn with_accept_encoding(mut self, value: &'a str) -> Self {
            self.accept_encoding = Some(value);
            self
        }

        pub(crate) fn query(&self) -> String {
            // dbg!(println!("{:#?}", &self.query));
            serde_qs::to_string(&self.query).unwrap()
        }

        pub(crate) fn headers(&self) -> HeaderMap {
            let mut headers = HeaderMap::new();
            if let Some(modified_since) = self.modified_since {
                let dt = modified_since.format(oss::GMT_DATE_FMT).to_string();
                headers.append(header::IF_MODIFIED_SINCE, dt.parse().unwrap());
            }
            if let Some(unmodified_since) = self.unmodified_since {
                let dt = unmodified_since.format(oss::GMT_DATE_FMT).to_string();
                headers.append(header::IF_UNMODIFIED_SINCE, dt.parse().unwrap());
            }
            if let Some(r#match) = self.r#match {
                headers.append(header::IF_MATCH, r#match.parse().unwrap());
            }
            if let Some(none_match) = self.none_match {
                headers.append(header::IF_MATCH, none_match.parse().unwrap());
            }
            if let Some(accept_encoding) = self.accept_encoding {
                headers.append(header::ACCEPT_ENCODING, accept_encoding.parse().unwrap());
            }
            headers
        }

        pub async fn send(&self) -> oss::Result<Bytes> {
            let query = self.query();
            let url = if !query.is_empty() {
                format!(
                    "{}/{}?{}",
                    self.client.options.base_url(),
                    self.object,
                    query
                )
            } else {
                format!("{}/{}", self.client.options.base_url(), self.object)
            };

            let headers = self.headers();

            let query_origin = urlencoding::decode(&query).unwrap();
            // println!("{}", query_origin);
            let mut task = self.client.request.task().resourse(&query_origin).url(&url);

            let task = if !headers.is_empty() {
                task.headers(headers)
            } else {
                task
            };

            let resp = task.send().await?;

            let result = oss::Data {
                status: resp.status,
                headers: resp.headers,
                data: resp.data,
            };
            Ok(result)
        }
    }

    #[allow(unused)]
    #[derive(Debug, Default)]
    pub struct CopyObjectBuilder<'a> {
        from_bucket: &'a str,
        form_object: &'a str,
        to_bucket: &'a str,
        to_object: &'a str,
        source_version_id: Option<&'a str>,
        version_id: Option<&'a str>,
        forbid_overwrite: Option<bool>,
        if_match: Option<&'a str>,
        if_none_match: Option<&'a str>,
        if_unmodified_since: Option<DateTime<Utc>>,
        if_modified_since: Option<DateTime<Utc>>,
        metadata_directive: (),
        encryption: Option<ServerSideEncryption>,
        enc_key_id: Option<&'a str>,
        object_acl: Option<ObjectACL>,
        storage_class: Option<StorageClass>,
        oss_tagging: Option<Tagging>,
        tagging_directive: (),
    }

    #[allow(unused)]
    #[derive(Debug)]
    pub struct DeleteObjectBuilder<'a> {
        client: &'a oss::Client<'a>,
        object: &'a str,
        version_id: Option<&'a str>,
    }

    #[allow(unused)]
    impl<'a> DeleteObjectBuilder<'a> {
        pub fn new(client: &'a oss::Client, object: &'a str) -> Self {
            Self {
                client,
                object,
                version_id: Default::default(),
            }
        }

        pub fn with_version_id(mut self, value: &'a str) -> Self {
            self.version_id = Some(value);
            self
        }

        pub async fn send(&self) -> oss::Result<()> {
            let url = match self.version_id {
                Some(version_id) => format!(
                    "{}/{}?versionId={}",
                    self.client.options.base_url(),
                    self.object,
                    version_id
                ),
                None => format!("{}/{}", self.client.options.base_url(), self.object),
            };

            // let query = {
            //     let url = Url::parse(&url).unwrap();
            //     url.query().unwrap().to_owned()
            // };

            let resp = self
                .client
                .request
                .task()
                .url(&url)
                // .resourse(query.as_str())
                .method(oss::Method::DELETE)
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

/// 基础操作
#[allow(non_snake_case)]
impl<'a> oss::Client<'a> {
    /// 调用PutObject接口上传文件（Object）
    pub fn PutObject(&self, object: &'a str) -> PutObjectBuilder {
        PutObjectBuilder::new(self, object)
    }

    /// GetObject接口用于获取某个文件（Object）。此操作需要对此Object具有读权限
    pub fn GetObject(&self, object: &'a str) -> GetObjectBuilder {
        GetObjectBuilder::new(self, object)
    }

    /// 调用CopyObject接口拷贝同一地域下相同或不同存储空间（Bucket）之间的文件（Object）
    pub async fn CopyObject(&self) {
        todo!()
    }

    /// 调用AppendObject接口用于以追加写的方式上传文件（Object）。通过AppendObject操
    /// 作创建的Object类型为Appendable Object，而通过PutObject上传的Object是Normal Object。
    pub async fn AppendObject(&self) {
        todo!()
    }

    /// 调用DeleteObject删除某个文件（Object）
    ///
    /// ### 注意事项
    ///
    /// - 要删除文件，您必须有oss:DeleteObject权限。要删除文件指定版本，您必须具有
    /// oss:DeleteObjectVersion权限。
    /// - 文件删除后无法恢复，请谨慎操作。关于删除文件的更多信息，请参见删除文件。
    /// - 无论要删除的Object是否存在，删除成功后均会返回204状态码。
    /// - 如果Object类型为软链接，使用DeleteObject接口只会删除该软链接。
    pub fn DeleteObject(&self, object: &'a str) -> DeleteObjectBuilder<'_> {
        DeleteObjectBuilder::new(self, object)
    }

    /// DeleteMultipleObjects接口用于删除同一个存储空间（Bucket）中的多个文件（Object）
    pub fn DeleteMultipleObjects() {
        todo!()
    }

    /// HeadObject接口用于获取某个文件（Object）的元信息
    pub async fn HeadObject(&self, object: &'a str) -> oss::Result<()> {
        let url = {
            let base_url = self.options.base_url();
            format!("{base_url}/{object}")
        };
        let resp = self
            .request
            .task()
            .url(&url)
            .method(oss::Method::HEAD)
            .send()
            .await
            .unwrap();

        let result = oss::Data {
            status: resp.status,
            headers: resp.headers,
            data: (),
        };
        Ok(result)
    }

    /// 调用GetObjectMeta接口获取一个文件（Object）的元数据信息
    ///
    /// 包括该Object的ETag、Size、LastModified信息，并且不返回该Object的内容。
    pub async fn GetObjectMeta(&self, object: &'a str) -> oss::Result<()> {
        let url = {
            let base_url = self.options.base_url();
            format!("{base_url}/{object}?objectMeta")
        };

        let resp = self
            .request
            .task()
            .url(&url)
            .method(oss::Method::HEAD)
            .resourse("objectMeta")
            .send()
            .await
            .unwrap();

        let result = oss::Data {
            status: resp.status,
            headers: resp.headers,
            data: (),
        };
        Ok(result)
    }

    /// 调用RestoreObject接口解冻归档类型、冷归档、深度冷归档类型的文件（Object）
    pub async fn RestoreObject(&self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::oss::{self, api::objects::stand::builder::GetObjectBuilder};
    use chrono::Utc;
    #[test]
    fn get_object_builder_arugments() {
        let option = oss::Options::default();
        let client = oss::Client::new(option);
        let builder = GetObjectBuilder::new(&client, "example/ex1.txt")
            .with_version_id("version123")
            .with_content_type("text/plain")
            .with_content_language("zh")
            .with_expires("expires")
            .with_cache_control("cache")
            .with_content_disposition("dis")
            .with_content_encoding("GZIP")
            .with_range((0, 100))
            .with_modified_since(Utc::now())
            .with_unmodified_since(Utc::now())
            .with_match("etag")
            .with_none_match("etag")
            .with_accept_encoding("text/plain");

        println!("  query: {}", builder.query());
        println!("headers: {:#?}", builder.headers());
    }
}
