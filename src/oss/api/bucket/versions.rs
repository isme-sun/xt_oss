use crate::oss::entities::version::VersioningConfiguration;
#[allow(unused)]
use crate::oss::{self, Client, Data, Method, Result};

use super::builders::PutBucketVersioningBuilder;

#[allow(non_snake_case)]
impl<'a> Client<'a> {
    pub fn PutBucketVersioning(&self) -> PutBucketVersioningBuilder {
        PutBucketVersioningBuilder::new(&self)
    }

    pub async fn GetBucketVersioning(&self) -> oss::Result<VersioningConfiguration> {
        let res = "versioning";
        let url = format!("{}/?{}", self.options.base_url(), res);
        let resp = self.request.task().url(&url).resourse(&res).send().await?;

        let content = String::from_utf8_lossy(&resp.data);
        let config: VersioningConfiguration = quick_xml::de::from_str(&content).unwrap();

        let result = oss::Data {
            status: resp.status,
            headers: resp.headers,
            data: config,
        };
        Ok(result)
    }

    pub fn ListObjectVersions() {
        todo!()
    }
}
