use crate::oss::Client;

use super::builders::ListBucketsBuilder;

#[allow(non_snake_case)]
/// 关于Region操作
impl<'a> Client<'a> {
    #[allow(private_interfaces)]
    pub fn ListBuckets(&self) -> ListBucketsBuilder {
        ListBucketsBuilder::new(&self)
    }
}
