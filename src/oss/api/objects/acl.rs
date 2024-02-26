use crate::oss;
use builders::{GetObjectAclBuilder, PutObjectACLBuilder};

pub mod builders {
    use crate::oss::{
        self,
        api::{self, ApiResponseFrom},
        entities::{acl::AccessControlPolicy, ObjectACL},
        http,
    };

    pub struct PutObjectACLBuilder<'a> {
        client: &'a oss::Client<'a>,
        object: &'a str,
        version_id: Option<&'a str>,
        acl: ObjectACL,
    }

    #[allow(unused)]
    impl<'a> PutObjectACLBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client, object: &'a str) -> Self {
            Self {
                client,
                object,
                version_id: None,
                acl: ObjectACL::Default,
            }
        }

        pub fn with_acl(mut self, acl: ObjectACL) -> Self {
            self.acl = acl;
            self
        }

        pub fn with_version_id(mut self, value: &'a str) -> Self {
            self.version_id = Some(value);
            self
        }

        pub async fn execute(&self) -> api::ApiResult {
            let mut res = format!("/{}/{}?{}", self.client.bucket(), self.object, "acl");
            let mut url = { format!("{}?{}", self.client.object_url(self.object), "acl") };
            if let Some(version_id) = self.version_id {
                res = format!("{}&versionId={}", res, version_id);
                url = format!("{}&versionId={}", url, version_id);
            }

            let mut headers = http::HeaderMap::new();
            headers.insert("x-oss-object-acl", self.acl.to_string().parse().unwrap());

            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_method(http::Method::PUT)
                .with_headers(headers)
                .with_resource(&res)
                .execute()
                .await?;
            Ok(ApiResponseFrom(resp).to_empty().await)
        }
    }

    pub struct GetObjectAclBuilder<'a> {
        client: &'a oss::Client<'a>,
        object: &'a str,
        version_id: Option<&'a str>,
    }

    impl<'a> GetObjectAclBuilder<'a> {
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

        pub async fn execute(&self) -> api::ApiResult<AccessControlPolicy> {
            let mut res = format!("/{}/{}?{}", self.client.bucket(), self.object, "acl");
            let mut url = { format!("{}?{}", self.client.object_url(self.object), "acl") };
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
            Ok(ApiResponseFrom(resp).to_type().await)
        }
    }
}

/// # 基础操作
#[allow(non_snake_case)]
impl<'a> oss::Client<'a> {
    /// 调用PutObjectACL接口修改文件`Object`的访问权限`ACL`。
    /// 此操作只有Bucket Owner有权限执行,且需对Object有读写权限。
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/putobjectacl)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_object_acl_put.rs)
    pub fn PutObjectACL(&self, object: &'a str) -> PutObjectACLBuilder {
        PutObjectACLBuilder::new(self, object)
    }

    /// 调用GetObjectACL接口获取存储空间`Bucket`下某个文件`Object`的访问权限`ACL`。
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/getobjectacl)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_object_acl_get.rs)
    pub fn GetObjectACL(&self, object: &'a str) -> GetObjectAclBuilder {
        GetObjectAclBuilder::new(self, object)
    }
}
