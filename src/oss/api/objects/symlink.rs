use crate::oss::{api::objects::symlink::builder::GetSymlinkBuilder, Client};

use self::builder::PutSymlinkBuilder;

pub mod builder {
    use reqwest::header::HeaderMap;

    use crate::oss::{
        self,
        entities::{ObjectACL, StorageClass},
    };

    #[allow(unused)]
    #[derive(Debug)]
    pub struct PutSymlinkBuilder<'a> {
        client: &'a oss::Client<'a>,
        object: &'a str,
        symlink_target: &'a str,
        forbid_overwrite: Option<bool>,
        object_acl: Option<ObjectACL>,
        storage_class: Option<StorageClass>,
    }

    #[allow(unused)]
    impl<'a> PutSymlinkBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client) -> Self {
            Self {
                client,
                object: Default::default(),
                symlink_target: Default::default(),
                forbid_overwrite: None,
                object_acl: None,
                storage_class: None,
            }
        }

        pub fn with_object(mut self, value: &'a str) -> Self {
            self.object = value;
            self
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

        pub async fn send(&self) -> oss::Result<()> {
            let query = "symlink";
            let url = format!(
                "{}/{}?{}",
                self.client.options.base_url(),
                self.object,
                query
            );

            println!("{}", url);

            let headers = self.headers();

            let resp = self
                .client
                .request
                .task()
                .url(&url)
                .headers(headers)
                .method(oss::Method::PUT)
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

        fn headers(&self) -> HeaderMap {
            let mut headers = HeaderMap::new();
            headers.insert("x-oss-symlink-target", self.symlink_target.parse().unwrap());
            headers
        }
    }

    #[allow(unused)]
    pub struct GetSymlinkBuilder<'a> {
        client: &'a oss::Client<'a>,
        object: &'a str,
        version_id: Option<&'a str>,
    }

    #[allow(unused)]
    impl<'a> GetSymlinkBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client) -> Self {
            Self {
                client,
                object: Default::default(),
                version_id: None,
            }
        }

        pub fn with_object(mut self, value: &'a str) -> Self {
            self.object = value;
            self
        }

        pub fn with_version_id(mut self, value: &'a str) -> Self {
            self.version_id = Some(value);
            self
        }

        pub async fn send(&self) -> oss::Result<()> {
            let query = "symlink";

            let url = if let Some(version_id) = self.version_id {
                format!(
                    "{}/{}?{}&versionId={}",
                    self.client.options.base_url(),
                    self.object,
                    query,
                    version_id
                )
            } else {
                format!(
                    "{}/{}?{}",
                    self.client.options.base_url(),
                    self.object,
                    query
                )
            };

            let resp = self
                .client
                .request
                .task()
                .url(&url)
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
}

/// 基础操作
#[allow(non_snake_case)]
impl<'a> Client<'a> {
    /// 使用Multipart Upload模式传输数据前，您必须先调用InitiateMultipartUpload接口来通知OSS
    /// 初始化一个Multipart Upload事件
    pub fn PutSymlink(&self) -> PutSymlinkBuilder<'_> {
        PutSymlinkBuilder::new(&self)
    }

    /// 初始化一个MultipartUpload后，调用UploadPart接口根据指定的Object名和uploadId来分块（Part）
    /// 上传数据
    pub fn GetSymlink(&self) -> GetSymlinkBuilder<'_> {
        GetSymlinkBuilder::new(&self)
    }
}
