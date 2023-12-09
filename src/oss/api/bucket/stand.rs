#[allow(unused)]
use crate::oss::{
    self,
    entities::{BucketInfo, BucketStat, ListBucketResult},
    Client, Data, Method, Result,
};

use super::builders::{ListObject2Builder, CreateBucketBuilder};

#[allow(non_snake_case)]
impl<'a> Client<'a> {

    pub fn PutBucket(&self) -> CreateBucketBuilder {
        CreateBucketBuilder::new(&self)
    }

    /// 调用DeleteBucket删除某个存储空间（Bucket）。
    /// - 只有Bucket的拥有者才有权限删除该Bucket。
    /// - 为了防止误删除的发生，OSS不允许删除一个非空的Bucket。
    pub fn DeleteBucket(&self) {
        todo!()
    }

    /// GetBucket (ListObjects)接口用于列举存储空间（Bucket）中所有文件（Object）的信息。
    pub fn ListObjects(&self) {
        todo!()
    }

    // ListObjectsV2（GetBucketV2）接口用于列举存储空间（Bucket）中所有文件（Object）的信息。
    #[allow(private_interfaces)]
    pub fn ListObjectsV2(&self) -> ListObject2Builder {
        ListObject2Builder::new(&self)
    }


    // 调用GetBucketInfo接口查看存储空间（Bucket）的相关信息。
    pub async fn GetBucketInfo(&self) -> Result<BucketInfo> {
        let res = "bucketInfo";
        let url = {
            let base_url = self.options.base_url();
            format!("{base_url}?{res}")
        };

        let resp = self
            .request
            .task()
            .url(&url)
            .resourse(&res)
            .send()
            .await
            .unwrap();

        let content = String::from_utf8_lossy(&resp.data);
        let bucket_info: BucketInfo = serde_xml_rs::from_str(&content).unwrap();
        let result = Data {
            status: resp.status,
            headers: resp.headers,
            data: bucket_info,
        };
        Ok(result)
    }

    /// GetBucketLocation接口用于查看存储空间（Bucket）的位置信息。
    /// 只有Bucket的拥有者才能查看Bucket的位置信息。
    pub fn GetBucketLocation(&self) {
        todo!()
    }

    pub async fn GetBucketStat(&self) -> Result<BucketStat> {
        let res = "stat";
        let url = {
            let base_url = self.options.base_url();
            format!("{base_url}?{res}")
        };
        let resp = self
            .request
            .task()
            .url(&url)
            .resourse(&res)
            .send()
            .await
            .unwrap();

        let content = String::from_utf8_lossy(&resp.data);
        let bucket_stat: BucketStat = serde_xml_rs::from_str(&content).unwrap();
        let result = Data {
            status: resp.status,
            headers: resp.headers,
            data: bucket_stat,
        };
        Ok(result)
    }

}
