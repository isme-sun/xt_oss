use crate::oss::api::bucket::builders::{DeleteBucketBuilder, ListObjectBuilder};
#[allow(unused)]
use crate::oss::{self, entities::BucketStat, Client, Data, Method, Result};

use super::builders::{
    BucketInfoBuilder, BucketLocationBuilder, BucketStatBuilder, CreateBucketBuilder,
    ListObject2Builder,
};

#[allow(non_snake_case)]
impl<'a> Client<'a> {
    pub fn PutBucket(&self) -> CreateBucketBuilder {
        CreateBucketBuilder::new(&self)
    }

    /// 调用DeleteBucket删除某个存储空间（Bucket）。
    /// - 只有Bucket的拥有者才有权限删除该Bucket。
    /// - 为了防止误删除的发生，OSS不允许删除一个非空的Bucket。
    #[allow(private_interfaces)]
    pub fn DeleteBucket(&self) -> DeleteBucketBuilder {
        DeleteBucketBuilder::new(&self)
    }

    /// GetBucket (ListObjects)接口用于列举存储空间（Bucket）中所有文件（Object）的信息。
    #[allow(private_interfaces)]
    pub fn ListObjects(&self) -> ListObjectBuilder {
        ListObjectBuilder::new(&self)
    }

    // ListObjectsV2（GetBucketV2）接口用于列举存储空间（Bucket）中所有文件（Object）的信息。
    #[allow(private_interfaces)]
    pub fn ListObjectsV2(&self) -> ListObject2Builder {
        ListObject2Builder::new(&self)
    }

    // 调用GetBucketInfo接口查看存储空间（Bucket）的相关信息。
    pub fn GetBucketInfo(&self) -> BucketInfoBuilder {
        BucketInfoBuilder::new(&self)
    }

    /// GetBucketLocation接口用于查看存储空间（Bucket）的位置信息。
    /// 只有Bucket的拥有者才能查看Bucket的位置信息。
    pub fn GetBucketLocation(&self) -> BucketLocationBuilder {
        BucketLocationBuilder::new(&self)
    }

    pub fn GetBucketStat(&self) -> BucketStatBuilder {
        BucketStatBuilder::new(&self)
    }
}
