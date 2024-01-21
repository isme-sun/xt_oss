use crate::oss::{self, entities::acl::AccessControlPolicy, Client};
use builders::PutObjectACLBuilder;

pub mod builders {
    use crate::oss::{self, entities::ObjectACL, header::HeaderMap};

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
                .resourse(query)
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
impl<'a> Client<'a> {
    /// 使用Multipart Upload模式传输数据前，您必须先调用InitiateMultipartUpload接口来通知OSS
    /// 初始化一个Multipart Upload事件
    pub fn PutObjectACL(&self, object: &'a str) -> PutObjectACLBuilder {
        PutObjectACLBuilder::new(self, object)
    }

    /// 初始化一个MultipartUpload后，调用UploadPart接口根据指定的Object名和uploadId来分块（Part）
    /// 上传数据
    pub async fn GetObjectACL(&self, object: &'a str) -> oss::Result<AccessControlPolicy> {
        let res = "acl";
        let url = {
            let base_url = &self.options.base_url();
            format!("{}/{}?{}", base_url, object, res)
        };

        let resp = self.request.task().url(&url).resourse(res).send().await?;
        let content = String::from_utf8_lossy(&resp.data);
        let data: AccessControlPolicy = quick_xml::de::from_str(&content).unwrap();

        let data = oss::Data {
            status: resp.status,
            headers: resp.headers,
            data,
        };

        Ok(data)
    }
}
