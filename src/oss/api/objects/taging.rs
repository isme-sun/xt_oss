use crate::oss::{
    self, api::objects::builders::DeleteObjectTaggingBuilder, entities::tag::Tagging, Client,
};

use super::builders::PutObjectTaggingBuilder;

/// 基础操作
#[allow(non_snake_case)]
impl<'a> Client<'a> {
    /// 使用Multipart Upload模式传输数据前，您必须先调用InitiateMultipartUpload接口来通知OSS
    /// 初始化一个Multipart Upload事件
    pub fn PutObjectTagging(&self, object: &'a str) -> PutObjectTaggingBuilder {
        PutObjectTaggingBuilder::new(self, object)
    }

    /// 初始化一个MultipartUpload后，调用UploadPart接口根据指定的Object名和uploadId来分块（Part）
    /// 上传数据
    pub async fn GetObjectTagging(&self, object: &'a str) -> oss::Result<Tagging> {
        let res = "tagging";
        let url = format!("{}/{}?{}", self.options.base_url(), object, res);

        let resp = self.request.task().url(&url).resourse(res).send().await?;

        let content = String::from_utf8_lossy(&resp.data);
        println!("{}", content);
        let tagging: Tagging = quick_xml::de::from_str(&content).unwrap();

        let result = oss::Data {
            status: resp.status,
            headers: resp.headers,
            data: tagging,
        };
        Ok(result)
    }

    /// 调用DeleteObjectTagging接口删除指定对象（Object）的标签（Tagging）信息。
    pub fn DeleteObjectTagging(&self, object: &'a str) -> DeleteObjectTaggingBuilder {
        DeleteObjectTaggingBuilder::new(self, object)
    }
}
