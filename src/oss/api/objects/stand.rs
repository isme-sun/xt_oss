use crate::oss::{self, Bytes, Client};

use super::builders::PutObjectBuilder;

/// 基础操作
#[allow(non_snake_case)]
impl<'a> Client<'a> {
	/// 调用PutObject接口上传文件（Object）
	pub fn PutObject(&self, object: &'a str) -> PutObjectBuilder {
		PutObjectBuilder::new(&self, object)
	}

	/// GetObject接口用于获取某个文件（Object）。此操作需要对此Object具有读权限
	pub async fn GetObject(&self, objectKey: &'a str) -> oss::Result<Bytes> {
		let url = {
			let base_url = self.options.base_url();
			format!("{base_url}/{objectKey}")
		};

		let resp = self.request.task().url(&url).send().await.unwrap();

		let result = oss::Data {
			status: resp.status,
			headers: resp.headers,
			data: resp.data,
		};
		Ok(result)
	}

	/// 调用CopyObject接口拷贝同一地域下相同或不同存储空间（Bucket）之间的文件（Object）
	pub async fn CopyObject(&self) {
		todo!()
	}

	/// 调用AppendObject接口用于以追加写的方式上传文件（Object）。通过AppendObject操
	/// 作创建的Object类型为Appendable Object，而通过PutObject上传的Object是Normal Object。
	pub async fn AppendObject(&self) {
		todo!()
	}

	pub async fn DeleteObject(&self) {
		todo!()
	}

	/// DeleteMultipleObjects接口用于删除同一个存储空间（Bucket）中的多个文件（Object）
	pub async fn DeleteMultipleObjects(&self) {
		todo!()
	}

	/// HeadObject接口用于获取某个文件（Object）的元信息
	pub async fn HeadObject(&self, object: &'a str) -> oss::Result<()> {
		let url = {
			let base_url = self.options.base_url();
			format!("{base_url}/{object}")
		};
		let resp = self
			.request
			.task()
			.url(&url)
			.method(oss::Method::HEAD)
			.send()
			.await
			.unwrap();

		let result = oss::Data {
			status: resp.status,
			headers: resp.headers,
			data: (),
		};
		Ok(result)
	}

	/// 调用GetObjectMeta接口获取一个文件（Object）的元数据信息
	///
	/// 包括该Object的ETag、Size、LastModified信息，并且不返回该Object的内容。
	pub async fn GetObjectMeta(&self, object: &'a str) -> oss::Result<()> {
		let url = {
			let base_url = self.options.base_url();
			format!("{base_url}/{object}?objectMeta")
		};

		let resp = self
			.request
			.task()
			.url(&url)
			.method(oss::Method::HEAD)
			.resourse("objectMeta")
			.send()
			.await
			.unwrap();

		let result = oss::Data {
			status: resp.status,
			headers: resp.headers,
			data: (),
		};
		Ok(result)
	}

	/// 调用RestoreObject接口解冻归档类型、冷归档、深度冷归档类型的文件（Object）
	pub async fn RestoreObject(&self) {
		todo!()
	}
}
