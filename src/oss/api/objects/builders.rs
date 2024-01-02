use chrono::{DateTime, Utc};
use std::collections::HashMap;

use crate::oss::{
    self,
    entities::{ObjectACL, ServerSideEncryption, StorageClass, Tag, TagSet, Tagging},
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
