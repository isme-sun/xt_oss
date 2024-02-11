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

        pub fn acl(mut self, acl: ObjectACL) -> Self {
            self.acl = acl;
            self
        }

        pub async fn execute(&self) -> api::ApiResult {
            let mut res = format!("/{}/{}?{}", self.client.bucket(), self.object, "acl");
            let mut url = { format!("{}?{}", self.client.object_url(self.object), res) };
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
            Ok(ApiResponseFrom(resp).as_empty().await)
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

        pub async fn execute(&self) -> api::ApiResult<AccessControlPolicy> {
            let mut res = format!("/{}/{}?{}", self.client.bucket(), self.object, "acl");
            let mut url = { format!("{}?{}", self.client.object_url(self.object), res) };
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
            Ok(ApiResponseFrom(resp).as_type().await)
        }
    }
}

/// 基础操作
#[allow(non_snake_case)]
impl<'a> oss::Client<'a> {
    /// 使用Multipart Upload模式传输数据前，您必须先调用InitiateMultipartUpload接口来通知OSS
    /// 初始化一个Multipart Upload事件
    pub fn PutObjectACL(&self, object: &'a str) -> PutObjectACLBuilder {
        PutObjectACLBuilder::new(self, object)
    }

    /// 初始化一个MultipartUpload后，调用UploadPart接口根据指定的Object名和uploadId来分块（Part）
    /// 上传数据
    pub async fn GetObjectACL(&self, object: &'a str) -> GetObjectAclBuilder {
        GetObjectAclBuilder::new(self, object)
    }
}
