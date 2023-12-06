use crate::oss::{self, Client};
use crate::oss::{
    arguments::ListBucketsQuery,
    entities::ListAllMyBucketsResult,
};

#[allow(non_snake_case)]
/// 关于Region操作
impl<'a> Client<'a> {
    // 调用ListBuckets（GetService）接口列举请求者拥有的所有存储空间（Bucket）。
    // 您还可以通过设置prefix、marker或者max-keys参数列举满足指定条件的存储空间。
    //
    pub async fn ListBuckets(
        &self,
        query: ListBucketsQuery,
    ) -> oss::Result<ListAllMyBucketsResult> {
        let url = {
            let base_url = self.options.root_url();
            format!("{}?{}", base_url, query)
        };
        let resp = self.request.task().url(&url).send().await.unwrap();
        let data: ListAllMyBucketsResult = resp.data.into();
        Ok(oss::Data {
            status: resp.status,
            headers: resp.headers,
            data,
        })
    }
}
