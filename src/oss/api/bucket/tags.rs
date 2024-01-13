use crate::oss::entities::tag::Tagging;
#[allow(unused)]
use crate::oss::{self, Client, Data, Method, Result};

use super::builders::{DeleteBucketTagsBuilder, PutBucketTagsBuilder};

#[allow(non_snake_case)]
impl<'a> Client<'a> {
    pub fn PutBucketTags(&self) -> PutBucketTagsBuilder {
        PutBucketTagsBuilder::new(self)
    }

    pub async fn GetBucketTags(&self) -> oss::Result<Tagging> {
        let res = "tagging";
        let url = format!("{}?{}", self.options.base_url(), res);

        let resp = self.request.task().url(&url).resourse(res).send().await?;

        let content = String::from_utf8_lossy(&resp.data);
        let tagging: Tagging = quick_xml::de::from_str(&content).unwrap();

        let result = oss::Data {
            status: resp.status,
            headers: resp.headers,
            data: tagging,
        };
        Ok(result)
    }

    pub fn DeleteBucketTags(&self) -> DeleteBucketTagsBuilder {
        DeleteBucketTagsBuilder::new(self)
    }
}
