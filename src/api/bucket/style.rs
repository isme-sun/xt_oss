#[allow(non_snake_case)]
impl OssClient {

	/// 调用PutStyle接口新增图片样式。一个图片样式中可以包含单个或多个图片处理参数
	pub async fn PutStyle() -> Result<()> {
		Ok(1)
	}

	/// 调用GetStyle接口查询某个Bucket下指定的样式信息
	pub async fn GetStyle() -> Result<()> {
		Ok(1)
	}

	/// 调用ListStyle接口查询某个Bucket下已创建的所有样式
	pub async fn ListStyle() -> Result<()> {
		Ok(1)
	}

	/// 调用DeleteStyle删除某个Bucket下指定的图片样式
	pub async fn DeleteStyle() -> Result<()> {
		Ok(1)
	}
}