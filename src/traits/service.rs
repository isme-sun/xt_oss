use async_trait::async_trait;
#[async_trait]
#[allow(non_snake_case)]
pub(crate) trait Service {
	async fn DescribeRegions();
}