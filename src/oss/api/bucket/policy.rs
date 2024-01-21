use crate::oss;

#[allow(non_snake_case)]
impl<'a> oss::Client<'a> {
    pub fn PutBucketPolicy() {
        todo!()
    }

    pub async fn GetBucketPolicy(&self) -> oss::Result<()> {
        let res = "policy";
        let url = format!("{}/?{}", self.options.base_url(), res);

        let resp = self.request.task().url(&url).resourse(res).send().await?;

        let content = String::from_utf8_lossy(&resp.data);

        println!("{}", content);

        let result = oss::Data {
            status: resp.status,
            headers: resp.headers,
            data: (),
        };
        Ok(result)
    }

    pub fn DeleteBucketPolicy() {
        todo!()
    }
}
