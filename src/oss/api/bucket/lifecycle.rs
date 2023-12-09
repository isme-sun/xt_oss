#[allow(unused)]
use crate::oss::{
    self,
    entities::{BucketInfo, BucketStat, ListBucketResult},
    Client, Data, Method, Result,
};

use super::builders::{CreateBucketBuilder, ListObject2Builder};

#[allow(non_snake_case)]
impl<'a> Client<'a> {
    /// 定的过期时间，自动转换与规则相匹配文件（Object）的存储类型或将其删除。
    #[allow(non_snake_case)]
    pub fn PutBucketLifecycle() {
        todo!()
    }

    /// 调用GetBucketLifecycle接口查看存储空间（Bucket）的生命周期规则（Lifecycle）。
    #[allow(non_snake_case)]
    pub fn GetBucketLifecycle() {
        todo!()
    }

    /// DeleteBucketLifecycle接口用于删除指定存储空间（Bucket）的生命周期规则。使用DeleteBucketLifecycle
    /// 接口删除指定Bucket所有的生命周期规则后，该Bucket中的文件（Object）不会被自动删除。只有Bucket的拥有者
    /// 才能删除该Bucket的生命周期规则。
    #[allow(non_snake_case)]
    pub fn DeleteBucketLifecycle() {
        todo!()
    }
}
