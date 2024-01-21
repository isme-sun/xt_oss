use crate::oss::{self, entities::acl::AccessControlPolicy, Client};

use super::builders::PutObjectACLBuilder;

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

		let resp = self.request.task().url(&url).resourse(&res).send().await?;
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
