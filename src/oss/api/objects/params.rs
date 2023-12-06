use crate::oss;

pub struct PutObjectParams<'a> {
    client: &'a oss::Client<'a>,
    object: &'a str,
    headers: oss::HeaderMap,
    content: oss::Bytes,
}

impl<'a> PutObjectParams<'a> {
    pub(crate) fn new(client: &'a oss::Client, object: &'a str) -> Self {
        Self {
            client,
            object,
            content: oss::Bytes::new(),
            headers: oss::HeaderMap::new(),
        }
    }

    pub fn content(mut self, content: oss::Bytes) -> Self {
        self.content = content;
        self
    }

    pub fn headers(mut self, headers: oss::HeaderMap) -> Self {
        self.headers = headers;
        self
    }

    pub async fn send(&self) -> oss::Result<()> {
        let url = {
            let base_url = &self.client.options.base_url();
            format!("{}/{}", base_url, &self.object)
        };

        let resp = self
            .client
            .request
            .task()
            .url(&url)
            .method(oss::Method::PUT)
            .body(self.content.to_owned())
            .send()
            .await
            .unwrap();

        let result = oss::Data {
            status: resp.status,
            headers: resp.headers,
            data: (),
        };
        Ok(result)
    }
}
