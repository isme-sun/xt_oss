use chrono::{DateTime, Utc};
use serde::Serialize;
use std::collections::HashMap;

use crate::oss::{
    self,
    entities::{ObjectACL, ServerSideEncryption, StorageClass, Tag, TagSet, Tagging},
    header,
    header::HeaderMap,
    Bytes,
};

pub struct PutObjectBuilder<'a> {
    client: &'a oss::Client<'a>,
    object: &'a str,
    headers: HeaderMap,
    content: oss::Bytes,
}

#[allow(unused)]
impl<'a> PutObjectBuilder<'a> {
    pub(crate) fn new(client: &'a oss::Client, object: &'a str) -> Self {
        Self {
            client,
            object,
            content: oss::Bytes::new(),
            headers: HeaderMap::new(),
        }
    }

    pub fn content(mut self, content: Bytes) -> Self {
        self.content = content;
        self
    }

    pub fn headers(mut self, headers: HeaderMap) -> Self {
        self.headers = headers;
        self
    }

    pub async fn send(&self) -> oss::Result<()> {
        let url = {
            let base_url = &self.client.options.base_url();
            format!("{}/{}", base_url, &self.object)
        };

        let resp = self
            .client
            .request
            .task()
            .url(&url)
            .method(oss::Method::PUT)
            .body(self.content.to_owned())
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

//////////////////////////////////////////////////////////////////////////
pub struct PutObjectACLBuilder<'a> {
    client: &'a oss::Client<'a>,
    object: &'a str,
    acl: ObjectACL,
}

#[allow(unused)]
impl<'a> PutObjectACLBuilder<'a> {
    pub(crate) fn new(client: &'a oss::Client, object: &'a str) -> Self {
        Self {
            client,
            object,
            acl: ObjectACL::Default,
        }
    }

    pub fn acl(mut self, acl: ObjectACL) -> Self {
        self.acl = acl;
        self
    }

    pub async fn send(&self) -> oss::Result<()> {
        let query = "acl";
        let url = {
            let base_url = &self.client.options.base_url();
            format!("{}/{}?{}", base_url, self.object, query)
        };

        let mut headers = HeaderMap::new();
        headers.insert("x-oss-object-acl", self.acl.to_string().parse().unwrap());

        let resp = self
            .client
            .request
            .task()
            .url(&url)
            .method(oss::Method::PUT)
            .headers(headers)
            .resourse(&query)
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

// ----------------------------------------------------------------------
#[allow(unused)]
pub struct PutObjectTaggingBuilder<'a> {
    client: &'a oss::Client<'a>,
    object: &'a str,
    tags: HashMap<&'a str, &'a str>,
}

#[allow(unused)]
impl<'a> PutObjectTaggingBuilder<'a> {
    pub fn new(client: &'a oss::Client, object: &'a str) -> Self {
        Self {
            client,
            object,
            tags: HashMap::new(),
        }
    }

    /// 添加tag
    pub fn add_tag(mut self, key: &'a str, value: &'a str) -> Self {
        self.tags.insert(key, value);
        self
    }

    /// 移除tag
    pub fn remove_tag(mut self, key: &'a str) -> Self {
        self.tags.remove(key);
        self
    }

    pub fn tagging(&self) -> Tagging {
        let mut tags: Vec<Tag> = Vec::new();
        for (key, value) in self.tags.clone() {
            tags.push(Tag {
                key: String::from(key),
                value: String::from(value),
            });
        }
        Tagging {
            tag_set: TagSet { tag: Some(tags) },
        }
    }

    pub fn tagging_xml(&self) -> String {
        quick_xml::se::to_string(&self.tagging()).unwrap()
    }

    pub async fn send(&self) -> oss::Result<()> {
        let res = "tagging";
        let url = format!("{}/{}?{}", self.client.options.base_url(), self.object, res);

        let data = oss::Bytes::from(self.tagging_xml());

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

#[allow(unused)]
pub struct DeleteObjectTaggingBuilder<'a> {
    client: &'a oss::Client<'a>,
    object: &'a str,
    version_id: Option<&'a str>,
}

#[allow(unused)]
impl<'a> DeleteObjectTaggingBuilder<'a> {
    pub fn new(client: &'a oss::Client, object: &'a str) -> Self {
        Self {
            client,
            object,
            version_id: None,
        }
    }

    pub fn version_id(mut self, value: &'a str) -> Self {
        self.version_id = Some(value);
        self
    }

    pub async fn send(&self) -> oss::Result<()> {
        let res = if let Some(version_id) = self.version_id {
            format!("tagging&versionId={}", version_id)
        } else {
            "tagging".to_string()
        };
        let url = { format!("{}/{}?{}", self.client.options.base_url(), self.object, res) };

        let resp = self
            .client
            .request
            .task()
            .url(&url)
            .method(oss::Method::DELETE)
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
}

///////////////////////////////////////////////////////////////////////////

#[allow(unused)]
#[derive(Debug, Default, Serialize)]
struct GetObjectBuilderArugments<'a> {
    #[serde(rename = "versionId", skip_serializing_if = "Option::is_none")]
    version_id: Option<&'a str>,
    #[serde(
        rename = "response-content-type",
        skip_serializing_if = "Option::is_none"
    )]
    content_type: Option<&'a str>,
    #[serde(
        rename = "response-content-type",
        skip_serializing_if = "Option::is_none"
    )]
    content_language: Option<&'a str>,
    #[serde(rename = "response-expires", skip_serializing_if = "Option::is_none")]
    expires: Option<&'a str>,
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
}

#[allow(unused)]
#[derive(Debug)]
struct GetObjectBuilder<'a> {
    client: &'a oss::Client<'a>,
    object: &'a str,
    range: Option<(u64, u64)>,
    modified_since: Option<DateTime<Utc>>,
    unmodified_since: Option<DateTime<Utc>>,
    r#match: Option<&'a str>,
    none_match: Option<&'a str>,
    accept_encoding: Option<&'a str>,
    arguments: GetObjectBuilderArugments<'a>,
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
            arguments: GetObjectBuilderArugments::default(),
        }
    }

    pub fn version_id(mut self, value: &'a str) -> Self {
        self.arguments.version_id = Some(value);
        self
    }
    pub fn content_type(mut self, value: &'a str) -> Self {
        self.arguments.content_type = Some(value);
        self
    }
    pub fn content_language(mut self, value: &'a str) -> Self {
        self
    }
    pub fn expires(mut self, value: &'a str) -> Self {
        self.arguments.expires = Some(value);
        self
    }
    pub fn cache_control(mut self, value: &'a str) -> Self {
        self.arguments.cache_control = Some(value);
        self
    }
    pub fn content_disposition(mut self, value: &'a str) -> Self {
        self.arguments.content_disposition = Some(value);
        self
    }
    pub fn content_encoding(mut self, value: &'a str) -> Self {
        self.arguments.content_encoding = Some(value);
        self
    }
    pub fn range(mut self, value: (u64, u64)) -> Self {
        self.range = Some(value);
        self
    }

    pub fn modified_since(mut self, value: DateTime<Utc>) -> Self {
        self.modified_since = Some(value);
        self
    }
    pub fn unmodified_since(mut self, value: DateTime<Utc>) -> Self {
        self.unmodified_since = Some(value);
        self
    }

    pub fn r#match(mut self, value: &'a str) -> Self {
        self.r#match = Some(value);
        self
    }

    pub fn none_match(mut self, value: &'a str) -> Self {
        self.none_match = Some(value);
        self
    }

    pub fn accept_encoding(mut self, value: &'a str) -> Self {
        self.accept_encoding = Some(value);
        self
    }

    pub(crate) fn query(&self) -> String {
        serde_qs::to_string(&self.arguments).unwrap()
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

    async fn send() -> oss::Result<Bytes> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::oss::{self, api::objects::builders::GetObjectBuilder};
    use chrono::Utc;
    #[test]
    fn get_object_builder_arugments() {
        let option = oss::Options::default();
        let client = oss::Client::new(option);
        let builder = GetObjectBuilder::new(&client, "example/ex1.txt")
            .version_id("version123")
            .content_type("text/plain")
            .content_language("zh")
            .expires("expires")
            .cache_control("cache")
            .content_disposition("dis")
            .content_encoding("GZIP")
            .range((0, 100))
            .modified_since(Utc::now())
            .unmodified_since(Utc::now())
            .r#match("etag")
            .none_match("etag")
            .accept_encoding("text/plain");

        println!("query: {}", builder.query());
        print!("headers: {:#?}", builder.headers());
    }
}
