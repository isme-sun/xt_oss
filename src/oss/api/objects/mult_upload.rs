// use crate::OssClient;
use crate::oss::{api::objects::mult_upload::builders::AbortMultipartUploadBuilder, Client};
use builders::{
    CompleteMultipartUploadBuilder, InitiateMultipartUploadBuilder, ListMultipartUploadsBuilder, UploadPartBuilder,
    UploadPartCopyBuilder,
};

use self::builders::ListPartsBuilder;

#[allow(unused)]
pub mod builders {
    use crate::oss::{self, api};

    pub struct InitiateMultipartUploadBuilder<'a> {
        client: &'a oss::Client<'a>,
    }

    impl<'a> InitiateMultipartUploadBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client) -> Self {
            Self { client }
        }

        pub async fn execute(&self) -> api::ApiResult {
            todo!()
        }
    }

    pub struct UploadPartBuilder<'a> {
        client: &'a oss::Client<'a>,
    }

    impl<'a> UploadPartBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client) -> Self {
            Self { client }
        }

        pub async fn execute(&self) -> api::ApiResult {
            todo!()
        }
    }

    pub struct UploadPartCopyBuilder<'a> {
        client: &'a oss::Client<'a>,
    }

    impl<'a> UploadPartCopyBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client) -> Self {
            Self { client }
        }

        pub async fn execute(&self) -> api::ApiResult {
            todo!()
        }
    }

    pub struct CompleteMultipartUploadBuilder<'a> {
        client: &'a oss::Client<'a>,
    }

    impl<'a> CompleteMultipartUploadBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client) -> Self {
            Self { client }
        }

        pub async fn execute(&self) -> api::ApiResult {
            todo!()
        }
    }

    pub struct AbortMultipartUploadBuilder<'a> {
        client: &'a oss::Client<'a>,
    }

    impl<'a> AbortMultipartUploadBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client) -> Self {
            Self { client }
        }

        pub async fn execute(&self) -> api::ApiResult {
            todo!()
        }
    }

    pub struct ListMultipartUploadsBuilder<'a> {
        client: &'a oss::Client<'a>,
    }

    impl<'a> ListMultipartUploadsBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client) -> Self {
            Self { client }
        }

        pub async fn execute(&self) -> api::ApiResult {
            todo!()
        }
    }

    pub struct ListPartsBuilder<'a> {
        client: &'a oss::Client<'a>,
    }

    impl<'a> ListPartsBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client) -> Self {
            Self { client }
        }

        pub async fn execute(&self) -> api::ApiResult {
            todo!()
        }
    }
}

/// 基础操作
#[allow(non_snake_case)]
impl<'a> Client<'a> {
    /// 使用Multipart Upload模式传输数据前，您必须先调用InitiateMultipartUpload接口来通知OSS初始化一
    /// 个Multipart Upload事件。
    ///
    /// - [official docs]()
    /// - [xtoss example]()
    pub fn InitiateMultipartUpload(&self) -> InitiateMultipartUploadBuilder {
        InitiateMultipartUploadBuilder::new(self)
    }

    /// 初始化一个MultipartUpload后，调用UploadPart接口根据指定的Object名和uploadId来分块（Part）上传数据。
    ///
    /// - [official docs]()
    /// - [xtoss example]()
    pub fn UploadPart(&self) -> UploadPartBuilder {
        UploadPartBuilder::new(self)
    }

    /// 通过在UploadPart请求的基础上增加一个请求头x-oss-copy-source来调用UploadPartCopy接口，实现从一个
    /// 已存在的Object中拷贝数据来上传一个Part。
    ///
    /// - [official docs]()
    /// - [xtoss example]()
    pub fn UploadPartCopy(&self) -> UploadPartCopyBuilder {
        UploadPartCopyBuilder::new(self)
    }

    /// 在将所有数据Part都上传完成后，您必须调用CompleteMultipartUpload接口来完成整个文件的分片上传。
    ///
    /// - [official docs]()
    /// - [xtoss example]()
    pub fn CompleteMultipartUpload(&self) -> CompleteMultipartUploadBuilder {
        CompleteMultipartUploadBuilder::new(self)
    }

    /// AbortMultipartUpload接口用于取消MultipartUpload事件并删除对应的Part数据。
    ///
    /// - [official docs]()
    /// - [xtoss example]()
    pub fn AbortMultipartUpload(&self) -> AbortMultipartUploadBuilder {
        AbortMultipartUploadBuilder::new(self)
    }

    /// 调用ListMultipartUploads接口列举所有执行中的Multipart Upload事件，即已经初始化但还未完成
    ///（Complete）或者还未中止（Abort）的Multipart Upload事件。
    ///
    /// - [official docs]()
    /// - [xtoss example]()
    pub fn ListMultipartUploads(&self) -> ListMultipartUploadsBuilder {
        ListMultipartUploadsBuilder::new(self)
    }

    /// ListParts接口用于列举指定Upload ID所属的所有已经上传成功Part。
    ///
    /// - [official docs]()
    /// - [xtoss example]()
    pub fn ListParts(&self) -> ListPartsBuilder {
        ListPartsBuilder::new(self)
    }
}
