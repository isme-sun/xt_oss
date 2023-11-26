use bytes::Bytes;
use reqwest::Method;

use crate::{util::Authorization, OssClient, OssData, OssResult};

#[allow(non_snake_case)]
impl OssClient {
    pub async fn GetObjectMeta(&self, objectKey: String) -> OssResult<Bytes> {
        let url = {
            let base_url = self.options.base_url();
            format!("{base_url}/{objectKey}?objectMeta")
        };

        let auth = Authorization {
            verb:Method::HEAD,
            bucket: Some(self.options.bucket.to_owned()),
            sub_res: Some("objectMeta".to_string()),
            object_key: Some(objectKey),
            ..Authorization::default()
        };

        let (_status, headers, data) = self.request(url, auth).await?;

        let ossData = OssData { headers, data };
        Ok(ossData)
    }
}
