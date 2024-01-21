use crate::oss::{self, entities::acl::AccessControlPolicy};

#[allow(unused)]
#[derive(Debug)]
pub struct PutBucketAclBuilder<'a> {
    client: &'a oss::Client<'a>,
    acl: oss::entities::OssAcl,
}

#[allow(unused)]
impl<'a> PutBucketAclBuilder<'a> {
    pub fn new(client: &'a oss::Client) -> Self {
        Self {
            client,
            acl: oss::entities::OssAcl::Private,
        }
    }

    pub fn acl(mut self, value: oss::entities::OssAcl) -> Self {
        self.acl = value;
        self
    }

    pub async fn send(&self) -> oss::Result<()> {
        let bucket = self.client.options.bucket;
        let res = "acl";
        let url = {
            format!(
                "{}://{}.{}/?{}",
                self.client.options.schema(),
                bucket,
                self.client.options.host(),
                res
            )
        };

        let mut headers = oss::header::HeaderMap::new();
        headers.insert("x-oss-acl", self.acl.to_string().parse().unwrap());

        let resp = self
            .client
            .request
            .task()
            .method(oss::Method::PUT)
            .headers(headers)
            .url(&url)
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

#[allow(non_snake_case)]
impl<'a> oss::Client<'a> {
    /// PutBucketAcl接口用于设置或修改存储空间（Bucket）的访问权限（ACL）。
    pub fn PutBucketAcl(&self) -> PutBucketAclBuilder {
        PutBucketAclBuilder::new(self)
    }

    /// GetBucketAcl接口用于获取某个存储空间（Bucket）的访问权限（ACL）。只有Bucket的拥有者才能获取Bucket的访问权限。
    pub async fn GetBucketAcl(&self) -> oss::Result<AccessControlPolicy> {
        let res = "acl";
        let url = {
            let base_url = self.options.base_url();
            format!("{base_url}?{res}")
        };

        let resp = self
            .request
            .task()
            .url(&url)
            .resourse(res)
            .send()
            .await
            .unwrap();

        let content = String::from_utf8_lossy(&resp.data);
        let cnames = quick_xml::de::from_str(&content).unwrap();
        let result = oss::Data {
            status: resp.status,
            headers: resp.headers,
            data: cnames,
        };
        Ok(result)
    }
}
