use crate::oss::Client;

use super::builders::DescribeRegionsBuilder;

#[allow(non_snake_case)]
/// 关于Service操作
impl<'a> Client<'a> {
    pub fn DescribeRegions(&self) -> DescribeRegionsBuilder {
        DescribeRegionsBuilder::new(&self)
    }
}
