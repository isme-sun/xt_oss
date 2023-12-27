use crate::oss::{self, entities::ObjectACL, Bytes};

pub struct PutObjectBuilder<'a> {
    client: &'a oss::Client<'a>,
    object: &'a str,
    headers: oss::HeaderMap,
    content: oss::Bytes,
}

#[allow(unused)]
impl<'a> PutObjectBuilder<'a> {
    pub(crate) fn new(client: &'a oss::Client, object: &'a str) -> Self {
        Self {
            client,
            object,
            content: oss::Bytes::new(),
            headers: oss::HeaderMap::new(),
        }
    }

    pub fn content(mut self, content: Bytes) -> Self {
        self.content = content;
        self
    }

    pub fn headers(mut self, headers: oss::HeaderMap) -> Self {
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

        let mut headers = oss::HeaderMap::new();
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
