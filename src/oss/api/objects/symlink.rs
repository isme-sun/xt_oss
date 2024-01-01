use crate::oss::Client;

/// 基础操作
#[allow(non_snake_case)]
impl<'a> Client<'a> {
    /// 使用Multipart Upload模式传输数据前，您必须先调用InitiateMultipartUpload接口来通知OSS
    /// 初始化一个Multipart Upload事件
    pub async fn PutSymlink(&self) {
        todo!()
    }

    /// 初始化一个MultipartUpload后，调用UploadPart接口根据指定的Object名和uploadId来分块（Part）
    /// 上传数据
    pub async fn GetSymlink(&self) {
        todo!()
    }
}
