use std::error::Error;
use crate::OssOptions;

pub struct OssClient {
    pub options: OssOptions,
}

/// OssClinet
impl OssClient {
    #[allow(dead_code)]
    pub fn builder(options: OssOptions) -> Self {
        OssClient { options }
    }

    /// 调用DescribeRegions接口查询所有支持地域或者指定地域对应的Endpoint信息，
    /// 包括外网Endpoint、内网Endpoint和传输加速Endpoint。
    pub async fn describe_regions(&self) -> Result<(), Box<dyn Error>> {
        let res = reqwest::get("http://httpbin.org/get?name=sjy").await?;

        println!("Status: {}", res.status());
        println!("Headers:\n{:#?}", res.headers());

        let body = res.text().await?;
        println!("Body:\n{}", body);
        Ok(())
    }

    /// 调用ListBuckets（GetService）接口列举请求者拥有的所有存储空间（Bucket）。
    /// 您还可以通过设置prefix、marker或者max-keys参数列举满足指定条件的存储空间。
    #[allow(non_snake_case)]
    pub fn ListBuckets(&self) {
        todo!()
    }

    /// 调用PutBucket接口创建存储空间（Bucket）。
    #[allow(non_snake_case)]
    pub fn PutBucket(&self) {
        todo!()
    }

    /// 调用DeleteBucket删除某个存储空间（Bucket）。
    /// - 只有Bucket的拥有者才有权限删除该Bucket。
    /// - 为了防止误删除的发生，OSS不允许删除一个非空的Bucket。
    #[allow(non_snake_case)]
    pub fn DeleteBucket(&self) {
        todo!()
    }

    /// GetBucket (ListObjects)接口用于列举存储空间（Bucket）中所有文件（Object）的信息。
    #[allow(non_snake_case)]
    pub fn GetBucket(&self) {
        todo!()
    }

    /// ListObjectsV2（GetBucketV2）接口用于列举存储空间（Bucket）中所有文件（Object）的信息。
    #[allow(non_snake_case)]
    pub fn ListObjectsV2(&self) {
        todo!()
    }

    /// 调用GetBucketInfo接口查看存储空间（Bucket）的相关信息。
    #[allow(non_snake_case)]
    pub fn GetBucketInfo(&self) {
        todo!()
    }

    /// GetBucketLocation接口用于查看存储空间（Bucket）的位置信息。
    /// 只有Bucket的拥有者才能查看Bucket的位置信息。
    #[allow(non_snake_case)]
    pub fn GetBucketLocation(&self) {
        todo!()
    }

    #[allow(non_snake_case)]
    pub fn GetBucketStat(&self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_work() {
        assert_eq!(1, 1);
    }
}
