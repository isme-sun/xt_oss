#[allow(non_snake_case)]
impl OssClient {
    /// 调用PutBucketReferer接口设置存储空间（Bucket）级别的Referer访问白名单以及黑名单
    pub async fn PutBucketReferer() -> Result<()> {
        Ok(1)
    }

    /// GetBucketReferer接口用于查看存储空间（Bucket）的防盗链（Referer）相关配置。
    pub async fn GetBucketReferer() -> Result<()> {
        Ok(1)
    }
}
